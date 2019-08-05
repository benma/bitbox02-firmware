// Copyright 2019 Shift Cryptosecurity AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#include <stdbool.h>
#include <stdint.h>
#include <string.h>

#include "trinary_input_char.h"
#include "ui_components.h"

#include "hardfault.h"
#include "screen.h"
#include "util.h"

#include <touch/gestures.h>
#include <ui/event.h>
#include <ui/event_handler.h>
#include <ui/fonts/arial_fonts.h>
#include <ui/ugui/ugui.h>
#include <ui/ui_util.h>

#ifndef TESTING
#include <driver_init.h>
#endif

#define EMPTY_CHAR '_'

static char ALPHABET[] = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
static char alphabet[] = "abcdefghijklmnopqrstuvwxyz";
static char digits[] = "0123456789";

typedef struct {
    // Can be NULL.
    const char* const* wordlist;
    size_t wordlist_size;
    // Only applies if wordlist != NULL: determines if a word from the wordlist was entered.
    bool can_confirm;
    // Mask user input with '*'?
    bool hide;
    void (*confirm_cb)(const char* string);

    // Internals follow.

    // Only applies if wordlist = NULL, in which case the user can select the keyboard mode.
    keyboard_mode_t input_mode;

    // Current state of input.
    size_t string_index;
    char string[INPUT_STRING_MAX_SIZE];

    component_t* trinary_char_component;
    component_t* confirm_gesture_component;
    component_t* left_arrow_component;
    component_t* keyboard_switch_component;
} data_t;

static void _cleanup(component_t* component)
{
    data_t* data = (data_t*)component->data;
    util_zero(data, sizeof(data_t));
    ui_util_component_cleanup(component);
}

static void _render(component_t* component)
{
    data_t* data = (data_t*)component->data;
    bool confirm_gesture_active =
        data->can_confirm && confirm_gesture_is_active(data->confirm_gesture_component);
    uint8_t string_x = 5;
    const uint8_t string_y = 30;
    UG_FontSelect(&font_font_a_11X12);
    for (size_t i = 0; i <= data->string_index; i++) {
        char chr;
        if (i == data->string_index) {
            chr = EMPTY_CHAR;
        } else if (i == data->string_index - 1 || !data->hide) {
            // Show character (or only last entered character in if input is hidden).
            chr = data->string[i];
        } else {
            chr = '*';
        }
        if (i == data->string_index && confirm_gesture_active) {
            // Don't show trailing char during confirm, to make it clear
            // that it is not part of the pw.
            continue;
        }
        UG_PutChar(chr, string_x, string_y, screen_front_color, screen_back_color, false);
        const uint8_t width = font_font_a_11X12.widths[chr - font_font_a_11X12.start_char];
        string_x += width + 2;
    }

    // Render sub-components
    if (data->can_confirm) {
        data->confirm_gesture_component->f->render(data->confirm_gesture_component);
    }
    if (!confirm_gesture_active) {
        data->left_arrow_component->f->render(data->left_arrow_component);
        if (data->keyboard_switch_component != NULL) {
            data->keyboard_switch_component->f->render(data->keyboard_switch_component);
        }
        data->trinary_char_component->f->render(data->trinary_char_component);
    }
}

static void _set_alphabet(component_t* trinary_input_string)
{
    data_t* data = (data_t*)trinary_input_string->data;
    component_t* trinary_char = data->trinary_char_component;
    if (data->wordlist != NULL) {
        // Restrict input charset based on the available words with.
        // E.g. if the user entered "act", and the wordlist contains "actor", "actress", "action",
        // the charset to select the next letter wil be "eio".
        // The wordlist is assumed to be sorted and only have 'a-z' characters.
        char charset[27] = {0};
        for (size_t word_idx = 0; word_idx < data->wordlist_size; word_idx++) {
            const char* word = data->wordlist[word_idx];
            bool is_prefix = strncmp(data->string, word, data->string_index) == 0;
            if (is_prefix) {
                if (strlen(word) > data->string_index) {
                    const char include = word[data->string_index];
                    if (strchr(charset, include) == NULL) {
                        charset[strlen(charset)] = include;
                    }
                }
            }
        }
        // Since wordlist is sorted, charset is sorted automatically.
        trinary_input_char_set_alphabet(trinary_char, charset);
    } else {
        // Otherwise set the input charset based on the user selected keyboard mode.
        switch (data->input_mode) {
        case DIGITS:
            trinary_input_char_set_alphabet(trinary_char, digits);
            break;
        case LOWER_CASE:
            trinary_input_char_set_alphabet(trinary_char, alphabet);
            break;
        case UPPER_CASE:
            trinary_input_char_set_alphabet(trinary_char, ALPHABET);
            break;
        default:
            break;
        }
    }
}

static void _set_can_confirm(component_t* trinary_input_string)
{
    data_t* data = (data_t*)trinary_input_string->data;
    if (data->wordlist == NULL) {
        data->can_confirm = true;
        return;
    }
    data->can_confirm = false;
    // Can only confirm if the entered word matches a word in the wordlist.
    for (size_t i = 0; i < data->wordlist_size; i++) {
        if (STREQ(data->wordlist[i], data->string)) {
            data->can_confirm = true;
            return;
        }
    }
}

static void _on_event(const event_t* event, component_t* component)
{
    data_t* data = (data_t*)component->data;

    if (event->id == EVENT_CONFIRM) {
        data->confirm_cb(data->string);
        return;
    }

    // Other gestures deactivated during confirming.
    if (confirm_gesture_is_active(data->confirm_gesture_component)) {
        return;
    }

    switch (event->id) {
    case EVENT_TOGGLE_ALPHANUMERIC:
        data->input_mode = (data->input_mode + 1) % NUM_INPUT_TYPES;
        _set_alphabet(component);
        break;
    case EVENT_BACKWARD:
        if (trinary_input_char_in_progress(data->trinary_char_component)) {
            _set_alphabet(component);
            break;
        }
        // Move cursor backward and display preceeding character
        if (data->string_index > 0) {
            data->string_index--;
            data->string[data->string_index] = '\0';
        }
        if (data->string_index > 0) {
            // Update the alphanumeric input mode
            if (strchr(alphabet, data->string[data->string_index])) {
                data->input_mode = LOWER_CASE;
            } else if (strchr(ALPHABET, data->string[data->string_index])) {
                data->input_mode = UPPER_CASE;
            } else if (strchr(digits, data->string[data->string_index])) {
                data->input_mode = DIGITS;
            }
            // Update keyboard_switch submodule mode in order to stay in sync
            event_t e = {
                .id = EVENT_UPDATE_ALPHANUMERIC,
                .data = &data->input_mode,
            };
            emit_event(&e);
        }
        _set_alphabet(component);
        _set_can_confirm(component);
        break;
    default:
        break;
    }

    if (data->string_index + 1 >= INPUT_STRING_MAX_SIZE) {
        event_t e;
        e.id = EVENT_CONFIRM;
        emit_event(&e);
    }
}

static void _letter_chosen(component_t* trinary_char, char chosen)
{
    component_t* trinary_input_string = trinary_char->parent;
    data_t* data = (data_t*)trinary_input_string->data;
    bool confirm_gesture_active = confirm_gesture_is_active(data->confirm_gesture_component);
    if (confirm_gesture_active) {
        _set_alphabet(trinary_input_string);
        return;
    }
    data->string[data->string_index] = chosen;
    data->string_index++;
    data->string[data->string_index] = '\0';
    _set_alphabet(trinary_input_string);
    _set_can_confirm(trinary_input_string);
}

static const component_functions_t component_functions = {
    .cleanup = _cleanup,
    .render = _render,
    .on_event = _on_event,
};

/********************************** Create Instance **********************************/

static component_t* _create(
    const char* const* wordlist,
    size_t wordlist_size,
    bool hide,
    void (*confirm_cb)(const char* input))
{
    component_t* component = malloc(sizeof(component_t));
    if (!component) {
        Abort("Error: malloc trinary_input_string");
    }
    data_t* data = malloc(sizeof(data_t));
    if (!data) {
        Abort("Error: malloc trinary_input_string data");
    }
    memset(component, 0, sizeof(component_t));
    memset(data, 0, sizeof(data_t));

    data->confirm_cb = confirm_cb;
    data->wordlist = wordlist;
    data->wordlist_size = wordlist_size;
    data->hide = hide;

    data->input_mode = LOWER_CASE;

    component->data = data;
    component->parent = NULL;
    component->f = &component_functions;
    component->dimension.width = SCREEN_WIDTH;
    component->dimension.height = SCREEN_HEIGHT;
    component->position.top = 0;
    component->position.left = 0;
    component->emit_without_release = true;

    data->left_arrow_component = left_arrow_create(top_slider, component);
    ui_util_add_sub_component(component, data->left_arrow_component);

    data->confirm_gesture_component = confirm_gesture_create(component);
    ui_util_add_sub_component(component, data->confirm_gesture_component);

    if (wordlist == NULL) {
        data->keyboard_switch_component = keyboard_switch_create(top_slider, component);
        ui_util_add_sub_component(component, data->keyboard_switch_component);
    }

    data->trinary_char_component = trinary_input_char_create("", _letter_chosen, component);
    ui_util_add_sub_component(component, data->trinary_char_component);
    _set_alphabet(component);
    _set_can_confirm(component);
    return component;
}

component_t* trinary_input_string_create_wordlist(
    const char* const* wordlist,
    size_t wordlist_size,
    void (*confirm_cb)(const char* input))
{
    if (wordlist == NULL) {
        Abort("trinary_input_string_\ncreate_wordlist");
    }
    return _create(wordlist, wordlist_size, false, confirm_cb);
}

component_t* trinary_input_string_create_password(void (*confirm_cb)(const char* input))
{
    return _create(NULL, 0, true, confirm_cb);
}

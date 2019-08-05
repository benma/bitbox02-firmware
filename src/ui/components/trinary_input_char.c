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

#include "trinary_input_char.h"

#include "button.h"

#include <hardfault.h>
#include <ui/ui_util.h>
#include <util.h>

typedef struct {
    void (*character_chosen_cb)(component_t*, char);
    component_t* button_left;
    char left_alphabet[27];
    char middle_alphabet[27];
    char right_alphabet[27];
    component_t* button_middle;
    component_t* button_right;
    char button_left_text[15];
    char button_middle_text[15];
    char button_right_text[15];

    bool in_progress;
} data_t;

static void _cleanup(component_t* component)
{
    ui_util_component_cleanup(component);
}

/**
 * Collects all component functions.
 */
static const component_functions_t _component_functions = {
    .cleanup = _cleanup,
    .render = ui_util_component_render_subcomponents,
    .on_event = ui_util_on_event_noop,
};

static void _insert_newline(const char* in, char* out, size_t out_len)
{
    size_t in_len = strlens(in);
    if (in_len > 4) {
        snprintf(out, out_len, "%.*s\n%s", (int)(in_len - 4), in, in + in_len - 4);
    } else {
        snprintf(out, out_len, "%.*s", (int)(in_len), in);
    }
}

static void _set_alphabet(component_t* component, const char* alphabet)
{
    data_t* data = (data_t*)component->data;
    size_t len = strlens(alphabet);
    if (len == 0) {
        return;
    }
    if (len == 1) {
        data->character_chosen_cb(component, alphabet[0]);
        data->in_progress = false;
        return;
    }
    trinary_input_char_set_alphabet(component, alphabet);
}

static void _left_selected(component_t* button)
{
    component_t* component = button->parent;
    data_t* data = (data_t*)component->data;
    _set_alphabet(component, data->left_alphabet);
    data->in_progress = true;
}

static void _middle_selected(component_t* button)
{
    component_t* component = button->parent;
    data_t* data = (data_t*)component->data;
    _set_alphabet(component, data->middle_alphabet);
    data->in_progress = true;
}

static void _right_selected(component_t* button)
{
    component_t* component = button->parent;
    data_t* data = (data_t*)component->data;
    _set_alphabet(component, data->right_alphabet);
    data->in_progress = true;
}

void trinary_input_char_set_alphabet(component_t* component, const char* alphabet_input)
{
    data_t* data = (data_t*)component->data;
    // copy so that alphabet_input can overlap with left_alphabet, middle_alphabet, right_alphabet
    // overwritten below.
    char alphabet[27];
    snprintf(alphabet, sizeof(alphabet), "%s", alphabet_input);

    size_t len = strlens(alphabet);
    size_t a = 0;
    size_t b = len / 3;
    size_t c = 2 * len / 3;
    size_t d = len;

    snprintf(data->left_alphabet, sizeof(data->left_alphabet), "%.*s", (int)(b - a), alphabet);
    snprintf(
        data->middle_alphabet, sizeof(data->middle_alphabet), "%.*s", (int)(c - b), alphabet + b);
    snprintf(
        data->right_alphabet, sizeof(data->right_alphabet), "%.*s", (int)(d - c), alphabet + c);

    _insert_newline(data->left_alphabet, data->button_left_text, sizeof(data->button_left_text));
    _insert_newline(
        data->middle_alphabet, data->button_middle_text, sizeof(data->button_middle_text));
    _insert_newline(data->right_alphabet, data->button_right_text, sizeof(data->button_right_text));

    button_update(data->button_left, data->button_left_text, _left_selected);
    button_update(data->button_middle, data->button_middle_text, _middle_selected);
    button_update(data->button_right, data->button_right_text, _right_selected);

    ui_util_position_left_bottom_offset(component, data->button_left, 0, 0);
    ui_util_position_left_bottom_offset(
        component,
        data->button_middle,
        SCREEN_WIDTH / 2 - data->button_middle->dimension.width / 2,
        0);
    ui_util_position_left_bottom_offset(
        component, data->button_right, SCREEN_WIDTH - data->button_right->dimension.width, 0);

    data->in_progress = false;
}

/********************************** Create Instance **********************************/

component_t* trinary_input_char_create(
    const char* alphabet,
    void (*character_chosen_cb)(component_t*, char),
    component_t* parent)
{
    data_t* data = malloc(sizeof(data_t));
    if (!data) {
        Abort("Error: malloc trinary char data");
    }
    memset(data, 0, sizeof(data_t));

    component_t* component = malloc(sizeof(component_t));
    if (!component) {
        Abort("Error: malloc trinary char");
    }
    memset(component, 0, sizeof(component_t));
    component->data = data;
    component->parent = parent;
    component->f = &_component_functions;
    component->dimension.width = SCREEN_WIDTH;
    component->dimension.height = SCREEN_HEIGHT;

    data->character_chosen_cb = character_chosen_cb;
    data->button_left = button_create("", bottom_slider, 0, NULL, component);
    ui_util_add_sub_component(component, data->button_left);

    data->button_middle = button_create("", bottom_slider, 0, NULL, component);
    ui_util_add_sub_component(component, data->button_middle);

    data->button_right = button_create("", bottom_slider, 0, NULL, component);
    ui_util_add_sub_component(component, data->button_right);
    trinary_input_char_set_alphabet(component, alphabet);

    return component;
}

bool trinary_input_char_in_progress(component_t* component)
{
    data_t* data = (data_t*)component->data;
    return data->in_progress;
}

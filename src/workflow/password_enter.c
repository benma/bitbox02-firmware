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

#include "password_enter.h"
#include "blocking.h"

#include <hardfault.h>
#include <keystore.h>
#include <ui/components/ui_components.h>
#include <ui/screen_process.h>
#include <ui/screen_stack.h>
#include <util.h>

#include <stdio.h>

static char _password[SET_PASSWORD_MAX_PASSWORD_LENGTH];

static void _pw_entered(const char* password)
{
    int snprintf_result = snprintf(_password, sizeof(_password), "%s", password);
    if (snprintf_result < 0 || snprintf_result >= (int)sizeof(_password)) {
        Abort("length mismatch");
    }
    workflow_blocking_unblock();
}

bool password_enter(const char* title, char* password_out)
{
    ui_screen_stack_push(entry_screen_create(title, workflow_blocking_unblock));
    if (!workflow_blocking_block()) {
        ui_screen_stack_pop();
        return false;
    }
    // ui_screen_stack_switch(set_password_create(_pw_entered));
    // const char* const word[4] = {"lol", "lal", "loll", "rofl"};
    size_t wordlist_size = keystore_get_bip39_wordlist_length();
    char* words[wordlist_size];
    for (size_t i = 0; i < wordlist_size; i++) {
        if (!keystore_get_bip39_word(i, &words[i])) {
        }
    }
    ui_screen_stack_switch(trinary_input_string_create_wordlist(
        (const char* const*)words, wordlist_size, _pw_entered));
    bool result = workflow_blocking_block();
    ui_screen_stack_pop();
    if (!result) {
        return false;
    }
    snprintf(password_out, SET_PASSWORD_MAX_PASSWORD_LENGTH, "%s", _password);
    util_zero(_password, sizeof(_password));
    return true;
}

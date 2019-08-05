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

#ifndef _TRINARY_INPUT_STRING_H_
#define _TRINARY_INPUT_STRING_H_

#include <ui/component.h>

#define INPUT_STRING_MAX_SIZE 10

/********************************** Create Instance **********************************/

/**
 * Creates a string input screen based on trinary input.
 * @param[in] The user can only confirm words from the wordlist. The available keyboard letters
 * adjust automatically so only words from the wordlist can be entered. Must be sorted, with all
 * words being lowercase 'a-z'.
 * @param[in] wordlist_size number of words in the wordlist.
 * @param[in] confirm_cb The callback that is called when the user entered the string.
 */
component_t* trinary_input_string_create_wordlist(
    const char* const* wordlist,
    size_t wordlist_size,
    void (*confirm_cb)(const char* input));

component_t* trinary_input_string_create_password(void (*confirm_cb)(const char* input));

#endif

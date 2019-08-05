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

#ifndef _TRINARY_INPUT_CHAR_H_
#define _TRINARY_INPUT_CHAR_H_

#include <screen.h>
#include <ui/component.h>

component_t* trinary_input_char_create(
    const char* alphabet,
    void (*character_chosen_cb)(component_t*, char),
    component_t* parent);

void trinary_input_char_set_alphabet(component_t* component, const char* alphabet_input);

bool trinary_input_char_in_progress(component_t*);
#endif

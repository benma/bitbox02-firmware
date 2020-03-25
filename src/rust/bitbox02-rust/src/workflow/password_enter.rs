// Copyright 2020 Shift Cryptosecurity AG
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

extern crate alloc;
use crate::bb02_async::option;
use alloc::boxed::Box;
use bitbox02::password::Password;
use core::pin::Pin;

async fn password_enter(title: &str, special_chars: bool, password_out: &mut Password) {
    let mut result: Pin<Box<Option<Password>>> = Box::pin(None);

    // The component will set the result password when the user entered it.
    let mut component =
        bitbox02::ui::trinary_input_string_create_password(title, special_chars, result.as_mut());

    bitbox02::ui::screen_stack_push(&mut component);
    // Wait for result to contain the password
    option(&result).await;
    bitbox02::ui::screen_stack_pop();

    let result: &Option<Password> = &*result;
    let result: Option<&Password> = result.as_ref();
    let result: &Password = result.unwrap();
    password_out.clone_from(&result);
    //password_out.set(result);
}

async fn asynctest() {
    let mut pw1 = Password::new();
    password_enter("hey", true, &mut pw1).await; //PasswordEnter::new("hey", true).await;
    bitbox02::screen_print_debug(
        unsafe { core::str::from_utf8_unchecked(&pw1.as_ref()[..]) },
        2000,
    );
    let mut pw2 = Password::new();
    password_enter("yo", false, &mut pw2).await;
    bitbox02::screen_print_debug(
        unsafe { core::str::from_utf8_unchecked(&pw2.as_ref()[..]) },
        2000,
    );
}

pub fn spin2() {
    extern crate alloc;
    let mut fut: crate::bb02_async::Future<()> = alloc::boxed::Box::pin(asynctest());
    loop {
        bitbox02::ui::screen_process();
        if crate::bb02_async::spin(&mut fut).is_ready() {
            break;
        }
    }
    // let mut task = make_task_from_workflow(unsafe { bitbox02_sys::workflow_stack_top() });
    // if spin(&mut task) {
    //     unsafe { bitbox02_sys::workflow_stack_stop_workflow() };
    // }
}

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
use core::pin::Pin;

pub use bitbox02::ui::{ConfirmParams as Params, SafeOption};
use core::marker::PhantomPinned;

/// Returns true if the user accepts, false if the user rejects.
pub async fn confirm(params: &Params<'_>) -> bool {
    let mut result: Pin<Box<SafeOption>> = Box::pin(SafeOption(None, PhantomPinned));
    // The component will set the result when the user accepted/rejected.
    let mut component = bitbox02::ui::confirm_create(&params, result.as_mut());

    bitbox02::ui::screen_stack_push(&mut component);
    // This does not work.
    // use core::ops::DerefMut;
    // *result.deref_mut() = SafeOption(Some(true), PhantomPinned);
    option(&result.0).await;
    bitbox02::ui::screen_stack_pop();

    result.0.unwrap()
}

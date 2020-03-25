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

use alloc::boxed::Box;
use core::pin::Pin;

/// Future is the core type for all polled futures in the BitBox02.
pub type Future<O> = Pin<Box<dyn core::future::Future<Output = O>>>;

/// A primitive poll invocation for a future, with no waking functionality.
pub fn spin<O>(future: &mut Future<O>) -> core::task::Poll<O> {
    let waker = async_task::waker_fn(|| {});
    let cx = &mut core::task::Context::from_waker(&waker);
    future.as_mut().poll(cx)
}

/// Implements the Option future, see `option()`.
pub struct AsyncOption<'a, O>(&'a Option<O>);

impl<O> core::future::Future for AsyncOption<'_, O> {
    type Output = ();
    fn poll(
        self: core::pin::Pin<&mut Self>,
        _cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        match self.0 {
            None => core::task::Poll::Pending,
            Some(_) => core::task::Poll::Ready(()),
        }
    }
}

/// Waits for an option to contain a value and returns a copy of that value.
/// E.g. `assert_eq!(option(&Some(42)).await, 42)`.
pub fn option<'a, O>(option: &'a Option<O>) -> AsyncOption<'a, O> {
    AsyncOption(&option)
}

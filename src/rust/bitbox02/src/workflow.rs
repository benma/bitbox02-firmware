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
use util::c_types::c_void;

use alloc::boxed::Box;
use core::future::Future;
use core::pin::Pin;

type Fut<O> = Pin<Box<dyn Future<Output = O>>>;

pub struct WorkflowFuture<Output> {
    workflow: *mut bitbox02_sys::_workflow_t,
    result: Option<Output>,
    initialized: bool,
}

impl<O> Drop for WorkflowFuture<O> {
    fn drop(&mut self) {
        if self.initialized {
            unsafe { bitbox02_sys::workflow_destroy(self.workflow) };
        }
    }
}

impl<Output: Copy + Unpin> Future for WorkflowFuture<Output> {
    type Output = Output;
    fn poll(
        mut self: core::pin::Pin<&mut Self>,
        _cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        if !self.initialized {
            unsafe {
                (*self.workflow).init.unwrap()(self.workflow);
            }
            self.initialized = true;
        }

        unsafe { (*self.workflow).spin.unwrap()(self.workflow) };
        match self.result {
            None => core::task::Poll::Pending,
            Some(r) => core::task::Poll::Ready(r),
        }
    }
}

impl<O> WorkflowFuture<O> {
    fn new() -> Pin<Box<Self>> {
        Box::pin(WorkflowFuture {
            workflow: core::ptr::null_mut(),
            result: None,
            initialized: false,
        })
    }
}

pub fn spin(future: &mut Fut<()>) -> bool {
    let waker = async_task::waker_fn(|| {});
    let cx = &mut core::task::Context::from_waker(&waker);
    match future.as_mut().poll(cx) {
        core::task::Poll::Ready(_) => true,
        core::task::Poll::Pending => false,
    }
}

// C-style including null terminator, as it is used in C.
// 150 corresponds to SET_PASSWORD_MAX_PASSWORD_LENGTH.
type Password = [u8; 150];

fn password_enter() -> Fut<Password> {
    extern "C" fn on_done_cb(password: *const u8, param: *mut c_void) {
        let out: Box<&mut Option<Password>> = unsafe { Box::from_raw(param as *mut _) };
        let mut password_out: Password = [0; 150];
        let len = password_out.len();
        password_out.copy_from_slice(unsafe { core::slice::from_raw_parts(password, len) });
        **out = Some(password_out);
    }

    let mut fut = WorkflowFuture::new();
    fut.workflow = unsafe {
        bitbox02_sys::password_enter(
            b"Test\0".as_ptr() as *const _,
            true,
            Some(on_done_cb),
            Box::into_raw(Box::new(&mut fut.result)) as *mut _,
        )
    };
    fut
}

async fn asynctest() {
    let pw1 = password_enter().await;
    unsafe { bitbox02_sys::screen_print_debug(pw1.as_ptr(), 2000) };
    let pw2 = password_enter().await;
    unsafe { bitbox02_sys::screen_print_debug(pw2.as_ptr(), 2000) };
}

pub fn spin2() {
    let mut fut: Fut<()> = Box::pin(asynctest());
    loop {
        unsafe { bitbox02_sys::screen_process() };
        if spin(&mut fut) {
            break;
        }
    }
    // let mut task = make_task_from_workflow(unsafe { bitbox02_sys::workflow_stack_top() });
    // if spin(&mut task) {
    //     unsafe { bitbox02_sys::workflow_stack_stop_workflow() };
    // }
}

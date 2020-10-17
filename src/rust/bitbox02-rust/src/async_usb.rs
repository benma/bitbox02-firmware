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

//! This module provides the executor for tasks that are spawned with an API request and deliver a
//! USB response.

extern crate alloc;

use crate::bb02_async::{option, spin as spin_task, AsyncOption, Task};
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::pin::Pin;
use core::task::{Context, Poll};

type UsbOut = Vec<u8>;
type UsbIn = Vec<u8>;

/// Describes the global state of an api query. The documentation of
/// the variants apply to the HWW stack, but have analogous meaning in
/// the U2F stack.
enum UsbTaskState<'a> {
    /// Waiting for a new query, nothing to do.
    Nothing,
    /// A query came in which launched a task, which is now running (e.g. user is entering a
    /// password). The option inside is `Some`, but is `None` for the brief period during which the
    /// task is polled. This is akin to popping off the task from the executor state and pushing it
    /// back in after it is polled, which is best practice (and often done via a message passing
    /// queue). This allows for the execeutor state to not be borrowed while the task is being
    /// executed, which allows the task itself to modify the executor state (otherwise we would have
    /// an illegal double-borrow of the state).
    Running(Option<Task<'a, UsbOut>>),
    /// The task has finished and written the result, so the USB response is available. We are now
    /// waiting for the client to fetch it (HWW_REQ_RETRY). For short-circuited or non-async api
    /// calls, the result might be returned immediately in response to HWW_REQ_NEW.
    ResultAvailable(UsbOut),
}

/// A safer version of UsbTaskState. RefCell so we cannot accidentally borrow illegally.
struct SafeUsbTaskState(RefCell<UsbTaskState<'static>>);

/// Safety: this implements Sync even though it is not thread safe. This is okay, as we
/// run only in a single thread in the BitBox02.
unsafe impl Sync for SafeUsbTaskState {}

struct BeSafe<A>(A);
unsafe impl<A> Sync for BeSafe<A> {}

/// Executor main state. Currently we only have at most one task at a time (usb api processing
/// task).
static USB_TASK_STATE: SafeUsbTaskState = SafeUsbTaskState(RefCell::new(UsbTaskState::Nothing));

enum OutFuture {
    ResultAvailable(UsbOut),
    ResultFetched,
}

struct OutFutureCell<'a>(&'a RefCell<OutFuture>);
impl core::future::Future for OutFutureCell<'_> {
    type Output = ();
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match *self.0.borrow() {
            OutFuture::ResultAvailable(_) => Poll::Pending,
            OutFuture::ResultFetched => Poll::Ready(()),
        }
    }
}

static IN_FUTURE: BeSafe<RefCell<Option<UsbIn>>> = BeSafe(RefCell::new(None));
static OUT_FUTURE: BeSafe<RefCell<OutFuture>> = BeSafe(RefCell::new(OutFuture::ResultFetched));

static mut RUNNING: bool = false;

pub async fn usb_in() -> UsbIn {
    option(&IN_FUTURE.0).await;
    unsafe {
        RUNNING = true;
    }
    IN_FUTURE.0.borrow_mut().take().unwrap()
}

pub async fn hww_main_loop() {
    loop {
        let in_bytes = usb_in().await;
        let out_bytes = crate::hww::process_packet(in_bytes).await;
        usb_out(out_bytes).await;
    }
}

static mut MEGALOOP_TASK: Option<Task<()>> = None;

/// Spawn a task to be spinned by the executor. This moves the state
/// from Nothing to Running.
///
/// *panics* - can only be called if the state is Nothing, otherwise panics.
pub fn spawn(usb_in: UsbIn) {
    let mut state = IN_FUTURE.0.borrow_mut();
    match *state {
        None => *state = Some(usb_in.to_vec()),
        Some(_) => panic!("previous task still in progress"),
    }
}

pub async fn usb_out(result: UsbOut) {
    {
        let mut state = OUT_FUTURE.0.borrow_mut();
        match &*state {
            ResultFetched => {
                *state = OutFuture::ResultAvailable(result);
                unsafe {
                    RUNNING = false;
                }
            }
            OutFuture::ResultAvailable(_) => panic!("already sending result"),
        }
    }
    OutFutureCell(&OUT_FUTURE.0).await
}

/// Spin the currently running task, if there is one. Otherwise do
/// nothing. This is supposed to be called from the mainloop.
///
/// If this spin finishes the task, the state is moved to
/// `ResultAvailable`, which contains the result.
pub fn spin() {
    match *unsafe { &mut MEGALOOP_TASK } {
        None => {
            unsafe { MEGALOOP_TASK = Some(Box::pin(hww_main_loop())) };
        }
        Some(ref mut t) => {
            spin_task(t);
        }
    }
    // // Pop task before polling, so that USB_TASK_STATE does not stay borrowed during the poll.
    // let mut popped_task = match *USB_TASK_STATE.0.borrow_mut() {
    //     // Illegal state, `None` is only valid during the poll.
    //     UsbTaskState::Running(None) => panic!("task not found"),
    //     // Get the task out, putting `None` in. This allows us to release the mutable borrow on the
    //     // state.
    //     UsbTaskState::Running(ref mut task @ Some(_)) => task.take(),
    //     // Nothing to do.
    //     _ => None,
    // };
    // if let Some(ref mut task) = popped_task {
    //     match spin_task(task) {
    //         Poll::Ready(result) => {
    //             *USB_TASK_STATE.0.borrow_mut() = UsbTaskState::ResultAvailable(result);
    //         }
    //         Poll::Pending => {
    //             // Not done yet, put the task back for execution.
    //             if let UsbTaskState::Running(ref mut task @ None) = *USB_TASK_STATE.0.borrow_mut() {
    //                 *task = popped_task;
    //             } else {
    //                 panic!("spin: illegal executor state");
    //             }
    //         }
    //     }
    // }
}

#[derive(PartialEq, Debug)]
pub enum CopyResponseErr {
    NotRunning,
    NotReady,
}

/// To be called in response to the client asking for the result of a
/// task.
///
/// If a result is available (state = ResultAvailable), this copies
/// the usb response to `dst` and moves the state to `Nothing`, and
/// returns the Ok(<number of bytes written>).
///
/// If there is no task running, returns `Err(true)` if a task is
/// pending and a response is expected in the future, or `Err(false)`
/// if no task is running.
pub fn copy_response(dst: &mut [u8]) -> Result<usize, CopyResponseErr> {
    if unsafe { RUNNING } {
        return Err(CopyResponseErr::NotReady);
    }
    let mut state = OUT_FUTURE.0.borrow_mut();
    match *state {
        OutFuture::ResultAvailable(ref response) => {
            let len = response.len();
            dst[..len].copy_from_slice(&response.as_ref());
            *state = OutFuture::ResultFetched;
            Ok(len)
        }
        OutFuture::ResultFetched => Err(CopyResponseErr::NotRunning),
    }
}

/// Cancel and drop a running task. Returns true if a task was cancelled, false if no task was
/// running.
pub fn cancel() -> bool {
    match *unsafe { &mut MEGALOOP_TASK } {
        None => false,
        Some(_) => {
            unsafe { MEGALOOP_TASK = Some(Box::pin(hww_main_loop())) };
            true
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;
    use std::prelude::v1::*;

    fn assert_panics<F: FnOnce() + std::panic::UnwindSafe>(f: F) {
        assert!(std::panic::catch_unwind(f).is_err());
    }

    /// Test spawning a task, spinning it, and getting the result.
    #[test]
    fn test_full_cycle() {
        async fn task(usb_in: UsbIn) -> UsbOut {
            assert_eq!(usb_in, [1, 2, 3].to_vec());
            [4, 5, 6, 7].to_vec()
        }
        fn assert_spawn_fails() {
            assert_panics(|| spawn(task, &[1, 2, 3]));
        }
        // repeated task processing ok
        for _ in 0..3 {
            let mut response = [0; 100];

            // No task running, can't copy response.
            assert_eq!(
                Err(CopyResponseErr::NotRunning),
                copy_response(&mut response)
            );

            spawn(task, &[1, 2, 3]);

            // Can't spawn: task already running.
            assert_spawn_fails();

            // Task not complete, can't copy response.
            assert_eq!(Err(CopyResponseErr::NotReady), copy_response(&mut response));

            spin();

            // Can't spawn: result not fetched yet
            assert_spawn_fails();

            // Response buffer too short.
            assert_panics(move || {
                let _ = copy_response(&mut response[..1]);
            });
            assert_eq!(Ok(4), copy_response(&mut response));
            // Response ok.
            assert_eq!(&response[..4], &[4, 5, 6, 7]);
        }
    }
}

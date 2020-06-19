// Copyright 2020 Shift Crypto AG
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

pub use bitbox02_sys::{Error, Request as CRequest, Response as CResponse};

extern crate alloc;
use alloc::vec::Vec;

use bitbox02_sys::{buffer_t, in_buffer_t, protobuf_decode, protobuf_encode};

pub enum Request {
    CRequest(CRequest),
}

/// Decodes a protobuf HWW request.
pub fn decode(input: Vec<u8>) -> Result<Request, ()> {
    let input = in_buffer_t {
        data: input.as_ptr(),
        len: input.len() as _,
    };
    unsafe {
        let mut req = core::mem::MaybeUninit::<CRequest>::zeroed();
        if !protobuf_decode(&input, req.as_mut_ptr()) {
            Err(())
        } else {
            Ok(Request::CRequest(req.assume_init()))
        }
    }
}

pub enum Response {
    Error,
    CResponse(alloc::boxed::Box<CResponse>),
}

impl Response {
    fn cresponse(self) -> alloc::boxed::Box<CResponse> {
        match self {
            Response::CResponse(r) => r,
            Response::Error => {
                let mut lol: alloc::boxed::Box<CResponse> = Default::default();
                lol.which_response = 2;
                lol.response.error = Error {
                    code: 0,
                    message: *b"lol\0...........................................................................................................................................................................................................................................................",
                };
                lol
            }
        }
    }
}

/// Encodes a protobuf HWW response.
/// `max_len` is the max size the resulting vector is allowed to be.
/// Aborts if the response does not fit.
pub fn encode(response: Response, max_len: usize) -> Vec<u8> {
    let mut output_vec = Vec::with_capacity(max_len);
    let mut output = buffer_t {
        data: output_vec.as_mut_ptr(),
        len: 0,
        max_len: output_vec.capacity() as _,
    };

    unsafe {
        protobuf_encode(&mut output, response.cresponse().as_ref());
        output_vec.set_len(output.len as _);
    };
    output_vec
}

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

extern crate alloc;
use alloc::vec::Vec;

use crate::hww::pb::hww::mod_Request::OneOfrequest;
use crate::hww::pb::hww::mod_Response::OneOfresponse;
use crate::hww::pb::hww::{Request, Response};
use crate::hww::pb::{bitbox02_system::*, hww, hww::*, random_number::*};

enum Error {
    InvalidInput,
    Memory,
    // Generic,
    UserAbort,
    // InvalidState,
    // Disabled,
    // Duplicate,
}

impl Error {
    fn to_protobuf(self) -> OneOfresponse {
        use alloc::string::ToString;

        let error = match self {
            Error::InvalidInput => hww::Error {
                code: 101,
                message: "invalid input".to_string(),
            },
            Error::Memory => hww::Error {
                code: 102,
                message: "memory".to_string(),
            },
            Error::UserAbort => hww::Error {
                code: 104,
                message: "aborted by the user".to_string(),
            },
        };
        OneOfresponse::error(error)
    }
}

fn protobuf_decode(input: &[u8]) -> Result<OneOfrequest, ()> {
    use quick_protobuf::{BytesReader, MessageRead};
    let mut reader = BytesReader::from_bytes(input);

    match Request::from_reader(&mut reader, input) {
        Ok(r) => Ok(r.request),
        Err(_) => Err(()),
    }
}

fn protobuf_encode(response: OneOfresponse) -> Vec<u8> {
    use quick_protobuf::{BytesWriter, MessageWrite, Writer};

    let mut rsp: Response = Default::default();
    rsp.response = response;

    // Same as (USB_DATA_MAX_LEN - 2) (1 byte reserved for HWW_RSP_* code, 1 byte for
    // OP_STATUS_SUCCESS).
    const MAX_OUT_LEN: usize = 7607;
    // -16 for encryption overhead
    let mut buf = [0u8; MAX_OUT_LEN - 16];
    let mut writer = Writer::new(BytesWriter::new(&mut buf));
    // Unwrap: if we don't have space for our response, we did someting wrong!
    rsp.write_message(&mut writer).unwrap();
    buf[..rsp.get_size()].to_vec()
}

async fn api_set_device_name(request: &SetDeviceNameRequest) -> Result<OneOfresponse, Error> {
    if request.name.as_bytes().len() > bitbox02::memory::DEVICE_NAME_MAX_SIZE {
        return Err(Error::InvalidInput);
    }

    use crate::workflow::confirm;
    let params = confirm::Params {
        title: "Name",
        body: &request.name,
        scrollable: true,
        ..Default::default()
    };
    if !confirm::confirm(&params).await {
        return Err(Error::UserAbort);
    }
    match bitbox02::memory::set_device_name(&request.name) {
        Ok(()) => Ok(OneOfresponse::success(Success {})),
        Err(()) => Err(Error::Memory),
    }
}

async fn api(request: &OneOfrequest) -> Option<OneOfresponse> {
    let response = match request {
        OneOfrequest::device_name(r) => api_set_device_name(&r).await,
        _ => return None,
    };
    match response {
        Ok(r) => Some(r),
        Err(err) => Some(err.to_protobuf()),
    }
}

pub async fn process(input: Vec<u8>) -> Vec<u8> {
    let request = match protobuf_decode(&input[..]) {
        Ok(request) => request,
        Err(_) => return protobuf_encode((Error::InvalidInput).to_protobuf()),
    };

    match api(&request).await {
        Some(r) => protobuf_encode(r),
        // Fall back to C commander for all api calls not handled in Rust.
        _ => bitbox02::commander::commander(input),
    }
}

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

use crate::hww::pb::hww;
use crate::hww::pb::hww::mod_Request::OneOfrequest;
use crate::hww::pb::hww::mod_Response::OneOfresponse;
use crate::hww::pb::hww::{Request, Response};

fn protobuf_encode(response: &Response) -> Vec<u8> {
    use quick_protobuf::{BytesWriter, MessageWrite, Writer};

    // Same as (USB_DATA_MAX_LEN - 2) (1 byte reserved for HWW_RSP_* code, 1 byte for
    // OP_STATUS_SUCCESS).
    const MAX_OUT_LEN: usize = 7607;
    // -16 for encryption overhead
    let mut buf = [0u8; MAX_OUT_LEN - 16];
    let mut writer = Writer::new(BytesWriter::new(&mut buf));
    // Unwrap: if we don't have space for our response, we did someting wrong!
    response.write_message(&mut writer).unwrap();
    buf[..response.get_size()].to_vec()
}

pub fn process(input: Vec<u8>) -> Vec<u8> {
    let request = {
        use quick_protobuf::{BytesReader, MessageRead};
        let mut reader = BytesReader::from_bytes(&input[..]);

        match Request::from_reader(&mut reader, &input[..]) {
            Ok(r) => r.request,
            Err(_) => {
                let mut rsp: Response = Default::default();
                rsp.response = OneOfresponse::success(hww::Success {});
                return protobuf_encode(&rsp);
            }
        }
    };

    match request {
        OneOfrequest::random_number(_) => {
            let mut rsp: Response = Default::default();
            rsp.response = OneOfresponse::success(hww::Success {});
            return protobuf_encode(&rsp);
        }
        _ => (),
    };
    bitbox02::commander::commander(input)
}

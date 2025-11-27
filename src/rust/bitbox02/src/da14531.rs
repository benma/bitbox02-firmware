// Copyright 2025 Shift Crypto AG
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

use crate::ringbuffer::RingBuffer;

pub fn set_name(name: &str, queue: &mut RingBuffer) {
    let name = crate::util::str_to_cstr_vec(name).unwrap();
    unsafe { bitbox02_sys::da14531_set_name(name.as_ptr(), &mut queue.inner as *mut _) };
}

pub fn set_product(product: &str, queue: &mut RingBuffer) {
    unsafe {
        bitbox02_sys::da14531_set_product(
            product.as_bytes().as_ptr() as *const _,
            product.len() as u16,
            &mut queue.inner,
        )
    }
}

pub fn power_down(queue: &mut RingBuffer) {
    unsafe {
        bitbox02_sys::da14531_power_down(&mut queue.inner as *mut _);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{vec, vec::Vec};

    const PACKET_TYPE_CTRL_DATA: u8 = 0xb4;
    const CTRL_CMD_DEVICE_NAME: u8 = 1;
    const CTRL_CMD_PRODUCT_STRING: u8 = 7;
    const SL_SOF: u8 = 0x7e;
    const SL_ESCAPE: u8 = 0x7d;
    const SL_XOR: u8 = 0x20;

    fn crc_reflect(mut data: u16, data_len: u8) -> u16 {
        let mut ret = data & 1;
        for _ in 1..data_len {
            data >>= 1;
            ret = (ret << 1) | (data & 1);
        }
        ret
    }

    fn crc_update(mut crc: u16, data: &[u8]) -> u16 {
        for &c in data {
            for i in 0..8 {
                let bit = (crc & 0x8000) ^ if (c & (1 << i)) != 0 { 0x8000 } else { 0 };
                crc <<= 1;
                if bit != 0 {
                    crc ^= 0x8005;
                }
                crc &= 0xffff;
            }
        }
        crc
    }

    fn crc_finalize(crc: u16) -> u16 {
        crc_reflect(crc, 16)
    }

    fn serial_link_format_byte(data: u8, out: &mut Vec<u8>) {
        match data {
            SL_SOF | SL_ESCAPE => {
                out.push(SL_ESCAPE);
                out.push(data ^ SL_XOR);
            }
            _ => out.push(data),
        }
    }

    fn format_ctrl_frame(payload: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        let mut crc = crc_update(0, &[PACKET_TYPE_CTRL_DATA]);

        out.push(SL_SOF);
        serial_link_format_byte(PACKET_TYPE_CTRL_DATA, &mut out);

        let payload_len = payload.len() as u16;
        let len_lo = (payload_len & 0xff) as u8;
        crc = crc_update(crc, &[len_lo]);
        serial_link_format_byte(len_lo, &mut out);

        let len_hi = (payload_len >> 8) as u8;
        crc = crc_update(crc, &[len_hi]);
        serial_link_format_byte(len_hi, &mut out);

        for &byte in payload {
            serial_link_format_byte(byte, &mut out);
        }

        crc = crc_update(crc, payload);
        crc = crc_finalize(crc);

        let mut crc_tmp = crc;
        for _ in 0..2 {
            let byte = (crc_tmp & 0xff) as u8;
            serial_link_format_byte(byte, &mut out);
            crc_tmp >>= 8;
        }

        out.push(SL_SOF);
        out
    }

    fn ringbuffer_contents(queue: &RingBuffer) -> Vec<u8> {
        let inner = &queue.inner;
        let mask = inner.size as usize;
        let mut idx = inner.read_index;
        let mut out = Vec::with_capacity((inner.write_index - inner.read_index) as usize);
        unsafe {
            let buf = core::slice::from_raw_parts(inner.buf, mask + 1);
            while idx != inner.write_index {
                out.push(buf[(idx as usize) & mask]);
                idx += 1;
            }
        }
        out
    }

    #[test]
    fn test_set_name_formats_frame() {
        let mut storage = [0u8; 256];
        let mut queue = RingBuffer::new(&mut storage);

        let name = "Hi~";
        set_name(name, &mut queue);

        let mut payload = vec![CTRL_CMD_DEVICE_NAME];
        payload.extend_from_slice(name.as_bytes());

        let expected = format_ctrl_frame(&payload);
        assert_eq!(queue.len() as usize, expected.len());
        assert_eq!(ringbuffer_contents(&queue), expected);
    }

    #[test]
    fn test_set_product_formats_frame() {
        let mut storage = [0u8; 256];
        let mut queue = RingBuffer::new(&mut storage);

        let product = "B}X";
        set_product(product, &mut queue);

        let mut payload = vec![CTRL_CMD_PRODUCT_STRING];
        payload.extend_from_slice(product.as_bytes());

        let expected = format_ctrl_frame(&payload);
        assert_eq!(queue.len() as usize, expected.len());
        assert_eq!(ringbuffer_contents(&queue), expected);
    }
}

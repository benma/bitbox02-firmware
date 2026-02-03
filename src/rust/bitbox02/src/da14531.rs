// SPDX-License-Identifier: Apache-2.0

use crate::ringbuffer::RingBuffer;

/// Set the product string of the BLE chip. The product string must be smaller than 64 bytes.
pub fn set_product(product: &str, queue: &mut RingBuffer) {
    let product = product.as_bytes();
    unsafe {
        bitbox02_sys::da14531_set_product(product.as_ptr(), product.len() as u16, &mut queue.inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate alloc;
    use alloc::vec;
    use alloc::vec::Vec;
    use bitbox_framed_serial_link::{ProtocolPacketType, protocol_format};

    const CTRL_CMD_PRODUCT_STRING: u8 = 7;

    fn drain(queue: &mut RingBuffer) -> Vec<u8> {
        let mut out = Vec::new();
        while queue.len() > 0 {
            out.push(queue.get().unwrap());
        }
        out
    }

    #[test]
    fn test_set_product() {
        let product = "foo bar";
        let mut buf = [0u8; 256];
        let mut queue = RingBuffer::new(&mut buf);

        set_product(product, &mut queue);

        let actual = drain(&mut queue);

        let mut payload = vec![CTRL_CMD_PRODUCT_STRING];
        payload.extend_from_slice(product.as_bytes());
        let mut expected = vec![0u8; 140];
        let expected_len = protocol_format(&mut expected, ProtocolPacketType::CtrlData, &payload);
        expected.truncate(expected_len);
        assert_eq!(actual, expected);
    }
}

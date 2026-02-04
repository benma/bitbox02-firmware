// SPDX-License-Identifier: Apache-2.0

pub fn write_poll(buf: &[u8; 64]) -> bool {
    unsafe { bitbox02_sys::hid_u2f_write_poll(buf.as_ptr() as *const _) }
}

pub fn read(buf: &mut [u8; 64]) -> bool {
    unsafe { bitbox02_sys::hid_u2f_read(buf as *mut _) }
}

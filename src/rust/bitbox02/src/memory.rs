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
// limitations under the License

pub const DEVICE_NAME_MAX_SIZE: usize = 63;

/// name.as_bytes() must be smaller or equal to `DEVICE_NAME_MAX_SIZE`.
pub fn set_device_name(name: &str) -> Result<(), ()> {
    if unsafe { bitbox02_sys::memory_set_device_name(crate::str_to_cstr_force!(name, 63).as_ptr()) }
    {
        Ok(())
    } else {
        Err(())
    }
}

pub fn is_initialized() -> bool {
    unsafe { bitbox02_sys::memory_is_initialized() }
}

pub fn is_mnemonic_passphrase_enabled() -> bool {
    unsafe { bitbox02_sys::memory_is_mnemonic_passphrase_enabled() }
}

pub fn get_attestation_pubkey_and_certificate(
    device_pubkey: &mut [u8; 64],
    certificate: &mut [u8; 64],
    root_pubkey_identifier: &mut [u8; 32],
) -> Result<(), ()> {
    match unsafe {
        bitbox02_sys::memory_get_attestation_pubkey_and_certificate(
            device_pubkey.as_mut_ptr(),
            certificate.as_mut_ptr(),
            root_pubkey_identifier.as_mut_ptr(),
        )
    } {
        true => Ok(()),
        false => Err(()),
    }
}

pub fn bootloader_hash(out: &mut [u8; 32]) {
    unsafe {
        bitbox02_sys::memory_bootloader_hash(out.as_mut_ptr());
    }
}

pub fn get_noise_static_private_key() -> Result<zeroize::Zeroizing<[u8; 32]>, ()> {
    let mut out = zeroize::Zeroizing::new([0u8; 32]);
    match unsafe { bitbox02_sys::memory_get_noise_static_private_key(out.as_mut_ptr()) } {
        true => Ok(out),
        false => Err(()),
    }
}

pub fn check_noise_remote_static_pubkey(pubkey: &[u8; 32]) -> bool {
    unsafe { bitbox02_sys::memory_check_noise_remote_static_pubkey(pubkey.as_ptr()) }
}

pub fn add_noise_remote_static_pubkey(pubkey: &[u8; 32]) -> Result<(), ()> {
    match unsafe { bitbox02_sys::memory_add_noise_remote_static_pubkey(pubkey.as_ptr()) } {
        true => Ok(()),
        false => Err(()),
    }
}

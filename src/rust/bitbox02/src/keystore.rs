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
use alloc::string::String;

use crate::password::Password;
use bitbox02_sys::keystore_error_t;

pub use bitbox02_sys::xpub_type_t;

pub const BIP39_WORDLIST_LEN: u16 = bitbox02_sys::BIP39_WORDLIST_LEN as u16;

pub fn is_locked() -> bool {
    unsafe { bitbox02_sys::keystore_is_locked() }
}

#[derive(Debug)]
pub enum Error {
    CannotUnlockBIP39,
    IncorrectPassword { remaining_attempts: u8 },
    Unknown,
}

pub fn unlock(password: &Password) -> Result<(), Error> {
    let mut remaining_attempts: u8 = 0;
    match unsafe { bitbox02_sys::keystore_unlock(password.as_cstr(), &mut remaining_attempts) } {
        keystore_error_t::KEYSTORE_OK => Ok(()),
        keystore_error_t::KEYSTORE_ERR_INCORRECT_PASSWORD => {
            Err(Error::IncorrectPassword { remaining_attempts })
        }
        keystore_error_t::KEYSTORE_ERR_MAX_ATTEMPTS_EXCEEDED => Err(Error::Unknown),
        keystore_error_t::KEYSTORE_ERR_GENERIC => Err(Error::Unknown),
    }
}

pub fn unlock_bip39(mnemonic_passphrase: &Password) -> Result<(), Error> {
    if unsafe { bitbox02_sys::keystore_unlock_bip39(mnemonic_passphrase.as_cstr()) } {
        Ok(())
    } else {
        Err(Error::CannotUnlockBIP39)
    }
}

pub fn create_and_store_seed(password: &Password, host_entropy: &[u8; 32]) -> bool {
    unsafe {
        bitbox02_sys::keystore_create_and_store_seed(password.as_cstr(), host_entropy.as_ptr())
    }
}

#[derive(Copy, Clone)]
struct ZeroizedMnemonic([u8; 256]);
impl core::default::Default for ZeroizedMnemonic {
    fn default() -> Self {
        ZeroizedMnemonic([0; 256])
    }
}
impl zeroize::DefaultIsZeroes for ZeroizedMnemonic {}

pub fn get_bip39_mnemonic() -> Result<zeroize::Zeroizing<String>, ()> {
    let mut mnemonic = zeroize::Zeroizing::new(ZeroizedMnemonic([0u8; 256]));
    match unsafe {
        bitbox02_sys::keystore_get_bip39_mnemonic(mnemonic.0.as_mut_ptr(), mnemonic.0.len() as _)
    } {
        false => Err(()),
        true => Ok(zeroize::Zeroizing::new(
            crate::util::str_from_null_terminated(&mnemonic.0[..])
                .unwrap()
                .into(),
        )),
    }
}

/// `idx` must be smaller than BIP39_WORDLIST_LEN.
pub fn get_bip39_word(idx: u16) -> Result<&'static str, ()> {
    let mut word: *mut u8 = core::ptr::null_mut();
    match unsafe { bitbox02_sys::keystore_get_bip39_word(idx, &mut word) } {
        false => Err(()),
        true => {
            let s = unsafe {
                let len = crate::util::strlen_ptr(word);
                core::slice::from_raw_parts(word, len as _)
            };
            Ok(core::str::from_utf8(&s[..]).unwrap())
        }
    }
}

pub fn encode_xpub_at_keypath(keypath: &[u32], xpub_type: xpub_type_t) -> Result<String, ()> {
    let mut xpub = [0u8; bitbox02_sys::XPUB_ENCODED_LEN as _];
    match unsafe {
        bitbox02_sys::keystore_encode_xpub_at_keypath(
            keypath.as_ptr(),
            keypath.len() as _,
            xpub_type,
            xpub.as_mut_ptr(),
            xpub.len() as _,
        )
    } {
        true => Ok(crate::util::str_from_null_terminated(&xpub[..])
            .unwrap()
            .into()),
        false => Err(()),
    }
}

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
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
pub use bitbox02_sys::backup_error_t as Error;
pub use bitbox02_sys::restore_error_t as RestoreError;

pub struct CheckData {
    pub id: String,
    pub name: String,
    // unix timestamp, UTC.
    pub birthdate: u32,
}

pub fn create(backup_create_timestamp: u32, seed_birthdate_timestamp: u32) -> Result<(), Error> {
    match unsafe { bitbox02_sys::backup_create(backup_create_timestamp, seed_birthdate_timestamp) }
    {
        Error::BACKUP_OK => Ok(()),
        err => Err(err),
    }
}

/// Returns the backup id, name and birthdate of the backup that matches the current keystore seed.
/// If none matches, `Err()` is returned.
pub fn check() -> Result<CheckData, Error> {
    let mut id = [0u8; 65];
    let mut name = [0u8; bitbox02_sys::MEMORY_DEVICE_NAME_MAX_LEN as _];
    let mut birthdate = 0u32;
    match unsafe { bitbox02_sys::backup_check(id.as_mut_ptr(), name.as_mut_ptr(), &mut birthdate) }
    {
        Error::BACKUP_OK => Ok(CheckData {
            id: crate::util::str_from_null_terminated(&id[..])
                .unwrap()
                .into(),
            name: crate::util::str_from_null_terminated(&name[..])
                .unwrap()
                .into(),
            birthdate,
        }),
        err => Err(err),
    }
}

pub struct Backup {
    pub seed: zeroize::Zeroizing<Vec<u8>>,
    pub name: String,
    pub seed_birthdate: u32,
}

pub fn restore_from_directory(dir: &str) -> Result<Box<Backup>, RestoreError> {
    let dir = crate::util::str_to_cstr_vec(dir).unwrap();
    let mut backup = core::mem::MaybeUninit::uninit();
    let mut backup_data = core::mem::MaybeUninit::uninit();
    match unsafe {
        bitbox02_sys::restore_from_directory(
            dir.as_ptr(),
            backup.as_mut_ptr(),
            backup_data.as_mut_ptr(),
        )
    } {
        RestoreError::RESTORE_OK => {
            let backup = unsafe { backup.assume_init() };
            let backup_data = unsafe { backup_data.assume_init() };
            Ok(Box::new(Backup {
                seed: zeroize::Zeroizing::new(
                    backup_data.seed[..backup_data.seed_length as usize].to_vec(),
                ),
                name: crate::util::str_from_null_terminated(
                    &backup.backup_v1.content.metadata.name,
                )
                .unwrap()
                .into(),
                seed_birthdate: backup_data.birthdate,
            }))
        }
        err => Err(err),
    }
}

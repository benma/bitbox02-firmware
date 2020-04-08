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

use alloc::vec::Vec;
use bitbox02::memory;
use core::cell::RefCell;

const OP_I_CAN_HAS_HANDSHAEK: u8 = b'h';
const OP_I_CAN_HAS_PAIRIN_VERIFICASHUN: u8 = b'v';
const OP_HER_COMEZ_TEH_HANDSHAEK: u8 = b'H';
const OP_NOISE_MSG: u8 = b'n';

/// To supply the randomness source to the noise crate.
pub enum BB02Random32 {}

impl bitbox02_noise::Random32 for BB02Random32 {
    fn mcu_32_bytes(out: &mut [u8; 32]) {
        bitbox02::random::mcu_32_bytes(out);
    }
}

/// A safer version of the noise state. RefCell so we cannot accidentally borrow illegally.
struct SafeNoiseState(RefCell<bitbox02_noise::State<BB02Random32>>);

/// Safety: this implements Sync even though it is not thread safe. This is okay, as we run only in
/// a single thread in the BitBox02.
unsafe impl Sync for SafeNoiseState {}

/// Global noise state, enforcing a proper handshake.
static NOISE_STATE: SafeNoiseState = SafeNoiseState(RefCell::new(bitbox02_noise::State::Nothing));

#[derive(Debug)]
pub struct Error;
impl core::convert::From<bitbox02_noise::Error> for Error {
    fn from(_error: bitbox02_noise::Error) -> Self {
        Error
    }
}
impl core::convert::From<()> for Error {
    fn from(_error: ()) -> Self {
        Error
    }
}

/// Process noise-encrypted messages:
/// - Enforce handshake
/// - Handle pairing verification
///   - Enforce pairing confirmation if the remote party is seen for the first time
///   - Remote party can invoke pairing confirmation anytime after the handshake
/// - Decrypt, process, encrypt
///
/// Returns Err if anything goes wrong:
/// - Invalid OP-code
/// - Noise message in the wrong state (e.g. handshake before init, etc.).
pub async fn noise(usb_in: Vec<u8>) -> Result<Vec<u8>, Error> {
    let mut state = NOISE_STATE.0.borrow_mut();

    match usb_in.split_first() {
        Some((&OP_I_CAN_HAS_HANDSHAEK, b"")) => {
            state.init(bitbox02_noise::Sensitive::from(
                memory::get_noise_static_private_key()?,
            ));
            Ok(Vec::new())
        }
        Some((&OP_HER_COMEZ_TEH_HANDSHAEK, rest)) => match state.handshake(rest)? {
            bitbox02_noise::HandshakeResult::Response(msg) => Ok(msg),
            bitbox02_noise::HandshakeResult::Done => {
                let already_verified =
                    memory::check_noise_remote_static_pubkey(&state.remote_static_pubkey()?);
                if already_verified {
                    state.set_pairing_verified()?;
                    Ok([0].to_vec())
                } else {
                    Ok([1].to_vec())
                }
            }
        },
        Some((&OP_I_CAN_HAS_PAIRIN_VERIFICASHUN, b"")) => {
            let hash = state.get_handshake_hash()?;
            if crate::workflow::pairing::confirm(&hash).await {
                state.set_pairing_verified()?;
                memory::add_noise_remote_static_pubkey(&state.remote_static_pubkey()?)?;
                Ok(Vec::new())
            } else {
                state.reset();
                Err(Error)
            }
        }
        Some((&OP_NOISE_MSG, encrypted_msg)) => {
            let decrypted_msg = state.decrypt(encrypted_msg)?;
            let response = bitbox02::commander::commander(decrypted_msg);
            Ok(state.encrypt(&response)?)
        }
        _ => Err(Error),
    }
}

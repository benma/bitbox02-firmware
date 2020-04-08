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

//! This module implements the state machine for establishing and using a noise channel.
//! The BitBox02 uses the following noise protocol config: Noise_XX_25519_ChaChaPoly_SHA256.

pub use crate::x25519::{PrivateKey, PublicKey};
use crate::x25519::{Random32, X25519};
use noise_rust_crypto::{sensitive::Sensitive, ChaCha20Poly1305, Sha256};

extern crate alloc;
use alloc::vec::Vec;

pub type HandshakeState<R> = noise_protocol::HandshakeState<X25519<R>, ChaCha20Poly1305, Sha256>;

pub type HandshakeHash = [u8; 32];

pub enum State<R: Random32> {
    /// Noise not in use yet.
    Nothing,
    Initialized(HandshakeState<R>),
    Ready {
        pairing_verification_required: bool,
        handshake_hash: HandshakeHash,
        remote_static_pubkey: PublicKey,
        send: noise_protocol::CipherState<ChaCha20Poly1305>,
        receive: noise_protocol::CipherState<ChaCha20Poly1305>,
    },
}

pub enum HandshakeResult {
    Response(Vec<u8>),
    Done,
}

#[derive(Debug)]
pub enum Error {
    PairingVerificationRequired,
    Noise,
    WrongState,
}

impl core::convert::From<Error> for () {
    fn from(_error: Error) -> Self {}
}

impl core::convert::From<noise_protocol::Error> for Error {
    fn from(_error: noise_protocol::Error) -> Self {
        Error::Noise
    }
}

impl<R: Random32> State<R> {
    pub fn new() -> Self {
        State::Nothing
    }

    pub fn reset(&mut self) {
        *self = State::Nothing;
    }

    pub fn init(&mut self, static_private_key: Sensitive<PrivateKey>) {
        let hs = HandshakeState::new(
            noise_protocol::patterns::noise_xx().clone(),
            false,
            &b"Noise_XX_25519_ChaChaPoly_SHA256"[..],
            Some(static_private_key),
            None,
            None,
            None,
        );
        *self = State::Initialized(hs);
    }

    pub fn handshake(&mut self, msg: &[u8]) -> Result<HandshakeResult, Error> {
        use core::convert::TryInto;
        match self {
            State::Initialized(handshake_state) => {
                let payload = handshake_state.read_message_vec(msg)?;

                if handshake_state.completed() {
                    let (receive, send) = handshake_state.get_ciphers();
                    let remote_static_pubkey = handshake_state.get_rs().ok_or(Error::Noise)?;
                    *self = State::Ready {
                        pairing_verification_required: true,
                        handshake_hash: handshake_state.get_hash().try_into().unwrap(),
                        remote_static_pubkey,
                        send,
                        receive,
                    };
                    return Ok(HandshakeResult::Done);
                }
                Ok(HandshakeResult::Response(
                    handshake_state.write_message_vec(&payload)?,
                ))
            }
            _ => Err(Error::WrongState),
        }
    }

    pub fn decrypt(&mut self, msg: &[u8]) -> Result<Vec<u8>, Error> {
        match self {
            State::Ready {
                pairing_verification_required: true,
                ..
            } => Err(Error::PairingVerificationRequired),

            State::Ready {
                pairing_verification_required: false,
                receive,
                ..
            } => match receive.decrypt_vec(&msg) {
                Ok(r) => Ok(r),
                Err(()) => Err(Error::Noise),
            },
            _ => Err(Error::WrongState),
        }
    }

    pub fn encrypt(&mut self, msg: &[u8]) -> Result<Vec<u8>, Error> {
        match self {
            State::Ready {
                pairing_verification_required: true,
                ..
            } => Err(Error::PairingVerificationRequired),

            State::Ready {
                pairing_verification_required: false,
                send,
                ..
            } => Ok(send.encrypt_vec(&msg)),
            _ => Err(Error::WrongState),
        }
    }

    pub fn get_handshake_hash(&self) -> Result<HandshakeHash, Error> {
        match self {
            State::Ready { handshake_hash, .. } => Ok(*handshake_hash),
            _ => Err(Error::WrongState),
        }
    }

    pub fn remote_static_pubkey(&self) -> Result<PublicKey, Error> {
        match self {
            State::Ready {
                remote_static_pubkey,
                ..
            } => Ok(*remote_static_pubkey),
            _ => Err(Error::WrongState),
        }
    }

    pub fn set_pairing_verified(&mut self) -> Result<(), Error> {
        match self {
            State::Ready {
                pairing_verification_required,
                ..
            } => {
                *pairing_verification_required = false;
                Ok(())
            }
            _ => Err(Error::WrongState),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    enum MockRandom32 {}
    impl Random32 for MockRandom32 {
        fn mcu_32_bytes(out: &mut [u8; 32]) {
            out.copy_from_slice(b"llllllllllllllllllllllllllllllll")
        }
    }

    #[test]
    pub fn test_lol() {
        use noise_protocol::DH;
        let i_host = X25519::<MockRandom32>::genkey();
        let i_bb02 = X25519::<MockRandom32>::genkey();

        let mut h_app = HandshakeState::<MockRandom32>::new(
            noise_protocol::patterns::noise_xx().clone(),
            true,
            &b"Noise_XX_25519_ChaChaPoly_SHA256"[..],
            Some(i_host),
            None,
            None,
            None,
        );
        let mut h_bb02 = State::<MockRandom32>::new();
        assert!(match h_bb02.handshake(&[]) {
            Err(Error::WrongState) => true,
            _ => false,
        });
        h_bb02.init(i_bb02);
        assert!(match h_bb02.handshake(&[]) {
            Err(Error::Noise) => true,
            _ => false,
        });
        let h_app_msg1 = h_app.write_message_vec(b"").unwrap();
        // let p = h_bb02.read_message_vec(&h_app_msg1).unwrap();
        // let h_bb02_msg1 = h_bb02.write_message_vec(&p).unwrap();
        if let HandshakeResult::Response(h_bb02_msg1) = h_bb02.handshake(&h_app_msg1).unwrap() {
            let p = h_app.read_message_vec(&h_bb02_msg1).unwrap();
            let h_app_msg2 = h_app.write_message_vec(&p).unwrap();
            if let HandshakeResult::Done { .. } = h_bb02.handshake(&h_app_msg2).unwrap() {}
        }
        // let (mut bb02_send, mut bb02_recv) = h_bb02.get_ciphers();
        let (mut app_send, mut app_recv) = h_app.get_ciphers();

        let encrypted = app_send.encrypt_vec(b"hallo");

        h_bb02.set_pairing_verified().unwrap();
        let decrypted = h_bb02.decrypt(&encrypted).unwrap();
        assert_eq!(decrypted, b"hallo");

        let encrypted = h_bb02.encrypt(b"yo").unwrap();
        let decrypted = app_recv.decrypt_vec(&encrypted).unwrap();
        assert_eq!(decrypted, b"yo");
    }
}

// Automatically generated rust module for 'perform_attestation.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use alloc::vec::Vec;
use alloc::borrow::Cow;
use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PerformAttestationRequest<'a> {
    pub challenge: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for PerformAttestationRequest<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.challenge = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PerformAttestationRequest<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.challenge == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.challenge).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.challenge != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.challenge))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct PerformAttestationResponse<'a> {
    pub bootloader_hash: Cow<'a, [u8]>,
    pub device_pubkey: Cow<'a, [u8]>,
    pub certificate: Cow<'a, [u8]>,
    pub root_pubkey_identifier: Cow<'a, [u8]>,
    pub challenge_signature: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for PerformAttestationResponse<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.bootloader_hash = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.device_pubkey = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.certificate = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.root_pubkey_identifier = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.challenge_signature = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PerformAttestationResponse<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.bootloader_hash == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.bootloader_hash).len()) }
        + if self.device_pubkey == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.device_pubkey).len()) }
        + if self.certificate == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.certificate).len()) }
        + if self.root_pubkey_identifier == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.root_pubkey_identifier).len()) }
        + if self.challenge_signature == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.challenge_signature).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.bootloader_hash != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.bootloader_hash))?; }
        if self.device_pubkey != Cow::Borrowed(b"") { w.write_with_tag(18, |w| w.write_bytes(&**&self.device_pubkey))?; }
        if self.certificate != Cow::Borrowed(b"") { w.write_with_tag(26, |w| w.write_bytes(&**&self.certificate))?; }
        if self.root_pubkey_identifier != Cow::Borrowed(b"") { w.write_with_tag(34, |w| w.write_bytes(&**&self.root_pubkey_identifier))?; }
        if self.challenge_signature != Cow::Borrowed(b"") { w.write_with_tag(42, |w| w.write_bytes(&**&self.challenge_signature))?; }
        Ok(())
    }
}


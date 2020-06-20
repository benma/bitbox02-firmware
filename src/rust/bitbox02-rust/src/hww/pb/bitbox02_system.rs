// Automatically generated rust module for 'bitbox02_system.proto' file

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
pub struct CheckSDCardRequest { }

impl<'a> MessageRead<'a> for CheckSDCardRequest {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for CheckSDCardRequest { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CheckSDCardResponse {
    pub inserted: bool,
}

impl<'a> MessageRead<'a> for CheckSDCardResponse {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.inserted = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for CheckSDCardResponse {
    fn get_size(&self) -> usize {
        0
        + if self.inserted == false { 0 } else { 1 + sizeof_varint(*(&self.inserted) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.inserted != false { w.write_with_tag(8, |w| w.write_bool(*&self.inserted))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DeviceInfoRequest { }

impl<'a> MessageRead<'a> for DeviceInfoRequest {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for DeviceInfoRequest { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct DeviceInfoResponse<'a> {
    pub name: Cow<'a, str>,
    pub initialized: bool,
    pub version: Cow<'a, str>,
    pub mnemonic_passphrase_enabled: bool,
    pub monotonic_increments_remaining: u32,
}

impl<'a> MessageRead<'a> for DeviceInfoResponse<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.initialized = r.read_bool(bytes)?,
                Ok(26) => msg.version = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(32) => msg.mnemonic_passphrase_enabled = r.read_bool(bytes)?,
                Ok(40) => msg.monotonic_increments_remaining = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DeviceInfoResponse<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.name == "" { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + if self.initialized == false { 0 } else { 1 + sizeof_varint(*(&self.initialized) as u64) }
        + if self.version == "" { 0 } else { 1 + sizeof_len((&self.version).len()) }
        + if self.mnemonic_passphrase_enabled == false { 0 } else { 1 + sizeof_varint(*(&self.mnemonic_passphrase_enabled) as u64) }
        + if self.monotonic_increments_remaining == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.monotonic_increments_remaining) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.name != "" { w.write_with_tag(10, |w| w.write_string(&**&self.name))?; }
        if self.initialized != false { w.write_with_tag(16, |w| w.write_bool(*&self.initialized))?; }
        if self.version != "" { w.write_with_tag(26, |w| w.write_string(&**&self.version))?; }
        if self.mnemonic_passphrase_enabled != false { w.write_with_tag(32, |w| w.write_bool(*&self.mnemonic_passphrase_enabled))?; }
        if self.monotonic_increments_remaining != 0u32 { w.write_with_tag(40, |w| w.write_uint32(*&self.monotonic_increments_remaining))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct InsertRemoveSDCardRequest {
    pub action: mod_InsertRemoveSDCardRequest::SDCardAction,
}

impl<'a> MessageRead<'a> for InsertRemoveSDCardRequest {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.action = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for InsertRemoveSDCardRequest {
    fn get_size(&self) -> usize {
        0
        + if self.action == bitbox02_system::mod_InsertRemoveSDCardRequest::SDCardAction::REMOVE_CARD { 0 } else { 1 + sizeof_varint(*(&self.action) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.action != bitbox02_system::mod_InsertRemoveSDCardRequest::SDCardAction::REMOVE_CARD { w.write_with_tag(8, |w| w.write_enum(*&self.action as i32))?; }
        Ok(())
    }
}

pub mod mod_InsertRemoveSDCardRequest {

use alloc::vec::Vec;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SDCardAction {
    REMOVE_CARD = 0,
    INSERT_CARD = 1,
}

impl Default for SDCardAction {
    fn default() -> Self {
        SDCardAction::REMOVE_CARD
    }
}

impl From<i32> for SDCardAction {
    fn from(i: i32) -> Self {
        match i {
            0 => SDCardAction::REMOVE_CARD,
            1 => SDCardAction::INSERT_CARD,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for SDCardAction {
    fn from(s: &'a str) -> Self {
        match s {
            "REMOVE_CARD" => SDCardAction::REMOVE_CARD,
            "INSERT_CARD" => SDCardAction::INSERT_CARD,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResetRequest { }

impl<'a> MessageRead<'a> for ResetRequest {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ResetRequest { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SetDeviceLanguageRequest<'a> {
    pub language: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for SetDeviceLanguageRequest<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.language = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SetDeviceLanguageRequest<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.language == "" { 0 } else { 1 + sizeof_len((&self.language).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.language != "" { w.write_with_tag(10, |w| w.write_string(&**&self.language))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SetDeviceNameRequest<'a> {
    pub name: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for SetDeviceNameRequest<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SetDeviceNameRequest<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.name == "" { 0 } else { 1 + sizeof_len((&self.name).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.name != "" { w.write_with_tag(10, |w| w.write_string(&**&self.name))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SetPasswordRequest<'a> {
    pub entropy: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for SetPasswordRequest<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.entropy = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SetPasswordRequest<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.entropy == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.entropy).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.entropy != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.entropy))?; }
        Ok(())
    }
}


// Automatically generated rust module for 'mnemonic.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use alloc::vec::Vec;
use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ShowMnemonicRequest { }

impl<'a> MessageRead<'a> for ShowMnemonicRequest {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ShowMnemonicRequest { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RestoreFromMnemonicRequest {
    pub timestamp: u32,
    pub timezone_offset: i32,
}

impl<'a> MessageRead<'a> for RestoreFromMnemonicRequest {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.timestamp = r.read_uint32(bytes)?,
                Ok(16) => msg.timezone_offset = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RestoreFromMnemonicRequest {
    fn get_size(&self) -> usize {
        0
        + if self.timestamp == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.timestamp) as u64) }
        + if self.timezone_offset == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.timezone_offset) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.timestamp != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.timestamp))?; }
        if self.timezone_offset != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.timezone_offset))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SetMnemonicPassphraseEnabledRequest {
    pub enabled: bool,
}

impl<'a> MessageRead<'a> for SetMnemonicPassphraseEnabledRequest {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.enabled = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for SetMnemonicPassphraseEnabledRequest {
    fn get_size(&self) -> usize {
        0
        + if self.enabled == false { 0 } else { 1 + sizeof_varint(*(&self.enabled) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.enabled != false { w.write_with_tag(8, |w| w.write_bool(*&self.enabled))?; }
        Ok(())
    }
}


// Automatically generated rust module for 'common.proto' file

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
pub struct PubResponse<'a> {
    pub pub_pb: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for PubResponse<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.pub_pb = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for PubResponse<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.pub_pb == "" { 0 } else { 1 + sizeof_len((&self.pub_pb).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.pub_pb != "" { w.write_with_tag(10, |w| w.write_string(&**&self.pub_pb))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RootFingerprintRequest { }

impl<'a> MessageRead<'a> for RootFingerprintRequest {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for RootFingerprintRequest { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RootFingerprintResponse<'a> {
    pub fingerprint: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for RootFingerprintResponse<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.fingerprint = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RootFingerprintResponse<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.fingerprint == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.fingerprint).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.fingerprint != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.fingerprint))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct XPub<'a> {
    pub depth: Cow<'a, [u8]>,
    pub parent_fingerprint: Cow<'a, [u8]>,
    pub child_num: u32,
    pub chain_code: Cow<'a, [u8]>,
    pub public_key: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for XPub<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.depth = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.parent_fingerprint = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.child_num = r.read_uint32(bytes)?,
                Ok(34) => msg.chain_code = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.public_key = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for XPub<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.depth == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.depth).len()) }
        + if self.parent_fingerprint == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.parent_fingerprint).len()) }
        + if self.child_num == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.child_num) as u64) }
        + if self.chain_code == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.chain_code).len()) }
        + if self.public_key == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.public_key).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.depth != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.depth))?; }
        if self.parent_fingerprint != Cow::Borrowed(b"") { w.write_with_tag(18, |w| w.write_bytes(&**&self.parent_fingerprint))?; }
        if self.child_num != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.child_num))?; }
        if self.chain_code != Cow::Borrowed(b"") { w.write_with_tag(34, |w| w.write_bytes(&**&self.chain_code))?; }
        if self.public_key != Cow::Borrowed(b"") { w.write_with_tag(42, |w| w.write_bytes(&**&self.public_key))?; }
        Ok(())
    }
}


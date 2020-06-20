// Automatically generated rust module for 'random_number.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use alloc::borrow::ToOwned;
use alloc::vec::Vec;
use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RandomNumberResponse {
    pub number: Vec<u8>,
}

impl<'a> MessageRead<'a> for RandomNumberResponse {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.number = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RandomNumberResponse {
    fn get_size(&self) -> usize {
        0
        + if self.number == vec![] { 0 } else { 1 + sizeof_len((&self.number).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.number != vec![] { w.write_with_tag(10, |w| w.write_bytes(&**&self.number))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RandomNumberRequest { }

impl<'a> MessageRead<'a> for RandomNumberRequest {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for RandomNumberRequest { }

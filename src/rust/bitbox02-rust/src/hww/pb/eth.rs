// Automatically generated rust module for 'eth.proto' file

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ETHCoin {
    ETH = 0,
    RopstenETH = 1,
    RinkebyETH = 2,
}

impl Default for ETHCoin {
    fn default() -> Self {
        ETHCoin::ETH
    }
}

impl From<i32> for ETHCoin {
    fn from(i: i32) -> Self {
        match i {
            0 => ETHCoin::ETH,
            1 => ETHCoin::RopstenETH,
            2 => ETHCoin::RinkebyETH,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ETHCoin {
    fn from(s: &'a str) -> Self {
        match s {
            "ETH" => ETHCoin::ETH,
            "RopstenETH" => ETHCoin::RopstenETH,
            "RinkebyETH" => ETHCoin::RinkebyETH,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ETHPubRequest<'a> {
    pub keypath: Vec<u32>,
    pub coin: ETHCoin,
    pub output_type: mod_ETHPubRequest::OutputType,
    pub display: bool,
    pub contract_address: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for ETHPubRequest<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.keypath = r.read_packed(bytes, |r, bytes| Ok(r.read_uint32(bytes)?))?,
                Ok(16) => msg.coin = r.read_enum(bytes)?,
                Ok(24) => msg.output_type = r.read_enum(bytes)?,
                Ok(32) => msg.display = r.read_bool(bytes)?,
                Ok(42) => msg.contract_address = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ETHPubRequest<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.keypath.is_empty() { 0 } else { 1 + sizeof_len(self.keypath.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + if self.coin == eth::ETHCoin::ETH { 0 } else { 1 + sizeof_varint(*(&self.coin) as u64) }
        + if self.output_type == eth::mod_ETHPubRequest::OutputType::ADDRESS { 0 } else { 1 + sizeof_varint(*(&self.output_type) as u64) }
        + if self.display == false { 0 } else { 1 + sizeof_varint(*(&self.display) as u64) }
        + if self.contract_address == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.contract_address).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_packed_with_tag(10, &self.keypath, |w, m| w.write_uint32(*m), &|m| sizeof_varint(*(m) as u64))?;
        if self.coin != eth::ETHCoin::ETH { w.write_with_tag(16, |w| w.write_enum(*&self.coin as i32))?; }
        if self.output_type != eth::mod_ETHPubRequest::OutputType::ADDRESS { w.write_with_tag(24, |w| w.write_enum(*&self.output_type as i32))?; }
        if self.display != false { w.write_with_tag(32, |w| w.write_bool(*&self.display))?; }
        if self.contract_address != Cow::Borrowed(b"") { w.write_with_tag(42, |w| w.write_bytes(&**&self.contract_address))?; }
        Ok(())
    }
}

pub mod mod_ETHPubRequest {

use alloc::vec::Vec;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OutputType {
    ADDRESS = 0,
    XPUB = 1,
}

impl Default for OutputType {
    fn default() -> Self {
        OutputType::ADDRESS
    }
}

impl From<i32> for OutputType {
    fn from(i: i32) -> Self {
        match i {
            0 => OutputType::ADDRESS,
            1 => OutputType::XPUB,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for OutputType {
    fn from(s: &'a str) -> Self {
        match s {
            "ADDRESS" => OutputType::ADDRESS,
            "XPUB" => OutputType::XPUB,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ETHSignRequest<'a> {
    pub coin: ETHCoin,
    pub keypath: Vec<u32>,
    pub nonce: Cow<'a, [u8]>,
    pub gas_price: Cow<'a, [u8]>,
    pub gas_limit: Cow<'a, [u8]>,
    pub recipient: Cow<'a, [u8]>,
    pub value: Cow<'a, [u8]>,
    pub data: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for ETHSignRequest<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.coin = r.read_enum(bytes)?,
                Ok(18) => msg.keypath = r.read_packed(bytes, |r, bytes| Ok(r.read_uint32(bytes)?))?,
                Ok(26) => msg.nonce = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.gas_price = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.gas_limit = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.recipient = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(58) => msg.value = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(66) => msg.data = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ETHSignRequest<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.coin == eth::ETHCoin::ETH { 0 } else { 1 + sizeof_varint(*(&self.coin) as u64) }
        + if self.keypath.is_empty() { 0 } else { 1 + sizeof_len(self.keypath.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + if self.nonce == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.nonce).len()) }
        + if self.gas_price == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.gas_price).len()) }
        + if self.gas_limit == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.gas_limit).len()) }
        + if self.recipient == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.recipient).len()) }
        + if self.value == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.value).len()) }
        + if self.data == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.data).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.coin != eth::ETHCoin::ETH { w.write_with_tag(8, |w| w.write_enum(*&self.coin as i32))?; }
        w.write_packed_with_tag(18, &self.keypath, |w, m| w.write_uint32(*m), &|m| sizeof_varint(*(m) as u64))?;
        if self.nonce != Cow::Borrowed(b"") { w.write_with_tag(26, |w| w.write_bytes(&**&self.nonce))?; }
        if self.gas_price != Cow::Borrowed(b"") { w.write_with_tag(34, |w| w.write_bytes(&**&self.gas_price))?; }
        if self.gas_limit != Cow::Borrowed(b"") { w.write_with_tag(42, |w| w.write_bytes(&**&self.gas_limit))?; }
        if self.recipient != Cow::Borrowed(b"") { w.write_with_tag(50, |w| w.write_bytes(&**&self.recipient))?; }
        if self.value != Cow::Borrowed(b"") { w.write_with_tag(58, |w| w.write_bytes(&**&self.value))?; }
        if self.data != Cow::Borrowed(b"") { w.write_with_tag(66, |w| w.write_bytes(&**&self.data))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ETHSignMessageRequest<'a> {
    pub coin: ETHCoin,
    pub keypath: Vec<u32>,
    pub msg: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for ETHSignMessageRequest<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.coin = r.read_enum(bytes)?,
                Ok(18) => msg.keypath = r.read_packed(bytes, |r, bytes| Ok(r.read_uint32(bytes)?))?,
                Ok(26) => msg.msg = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ETHSignMessageRequest<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.coin == eth::ETHCoin::ETH { 0 } else { 1 + sizeof_varint(*(&self.coin) as u64) }
        + if self.keypath.is_empty() { 0 } else { 1 + sizeof_len(self.keypath.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + if self.msg == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.msg).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.coin != eth::ETHCoin::ETH { w.write_with_tag(8, |w| w.write_enum(*&self.coin as i32))?; }
        w.write_packed_with_tag(18, &self.keypath, |w, m| w.write_uint32(*m), &|m| sizeof_varint(*(m) as u64))?;
        if self.msg != Cow::Borrowed(b"") { w.write_with_tag(26, |w| w.write_bytes(&**&self.msg))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ETHSignResponse<'a> {
    pub signature: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for ETHSignResponse<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.signature = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ETHSignResponse<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.signature == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.signature).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.signature != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.signature))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ETHRequest<'a> {
    pub request: mod_ETHRequest::OneOfrequest<'a>,
}

impl<'a> MessageRead<'a> for ETHRequest<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.request = mod_ETHRequest::OneOfrequest::pub_pb(r.read_message::<ETHPubRequest>(bytes)?),
                Ok(18) => msg.request = mod_ETHRequest::OneOfrequest::sign(r.read_message::<ETHSignRequest>(bytes)?),
                Ok(26) => msg.request = mod_ETHRequest::OneOfrequest::sign_msg(r.read_message::<ETHSignMessageRequest>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ETHRequest<'a> {
    fn get_size(&self) -> usize {
        0
        + match self.request {
            mod_ETHRequest::OneOfrequest::pub_pb(ref m) => 1 + sizeof_len((m).get_size()),
            mod_ETHRequest::OneOfrequest::sign(ref m) => 1 + sizeof_len((m).get_size()),
            mod_ETHRequest::OneOfrequest::sign_msg(ref m) => 1 + sizeof_len((m).get_size()),
            mod_ETHRequest::OneOfrequest::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.request {            mod_ETHRequest::OneOfrequest::pub_pb(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            mod_ETHRequest::OneOfrequest::sign(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_ETHRequest::OneOfrequest::sign_msg(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            mod_ETHRequest::OneOfrequest::None => {},
    }        Ok(())
    }
}

pub mod mod_ETHRequest {

use alloc::vec::Vec;
use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfrequest<'a> {
    pub_pb(ETHPubRequest<'a>),
    sign(ETHSignRequest<'a>),
    sign_msg(ETHSignMessageRequest<'a>),
    None,
}

impl<'a> Default for OneOfrequest<'a> {
    fn default() -> Self {
        OneOfrequest::None
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ETHResponse<'a> {
    pub response: mod_ETHResponse::OneOfresponse<'a>,
}

impl<'a> MessageRead<'a> for ETHResponse<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.response = mod_ETHResponse::OneOfresponse::pub_pb(r.read_message::<common::PubResponse>(bytes)?),
                Ok(18) => msg.response = mod_ETHResponse::OneOfresponse::sign(r.read_message::<ETHSignResponse>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ETHResponse<'a> {
    fn get_size(&self) -> usize {
        0
        + match self.response {
            mod_ETHResponse::OneOfresponse::pub_pb(ref m) => 1 + sizeof_len((m).get_size()),
            mod_ETHResponse::OneOfresponse::sign(ref m) => 1 + sizeof_len((m).get_size()),
            mod_ETHResponse::OneOfresponse::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.response {            mod_ETHResponse::OneOfresponse::pub_pb(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            mod_ETHResponse::OneOfresponse::sign(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_ETHResponse::OneOfresponse::None => {},
    }        Ok(())
    }
}

pub mod mod_ETHResponse {

use alloc::vec::Vec;
use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfresponse<'a> {
    pub_pb(common::PubResponse<'a>),
    sign(ETHSignResponse<'a>),
    None,
}

impl<'a> Default for OneOfresponse<'a> {
    fn default() -> Self {
        OneOfresponse::None
    }
}

}


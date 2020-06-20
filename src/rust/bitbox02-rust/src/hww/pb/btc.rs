// Automatically generated rust module for 'btc.proto' file

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
pub enum BTCCoin {
    BTC = 0,
    TBTC = 1,
    LTC = 2,
    TLTC = 3,
}

impl Default for BTCCoin {
    fn default() -> Self {
        BTCCoin::BTC
    }
}

impl From<i32> for BTCCoin {
    fn from(i: i32) -> Self {
        match i {
            0 => BTCCoin::BTC,
            1 => BTCCoin::TBTC,
            2 => BTCCoin::LTC,
            3 => BTCCoin::TLTC,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for BTCCoin {
    fn from(s: &'a str) -> Self {
        match s {
            "BTC" => BTCCoin::BTC,
            "TBTC" => BTCCoin::TBTC,
            "LTC" => BTCCoin::LTC,
            "TLTC" => BTCCoin::TLTC,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BTCOutputType {
    UNKNOWN = 0,
    P2PKH = 1,
    P2SH = 2,
    P2WPKH = 3,
    P2WSH = 4,
}

impl Default for BTCOutputType {
    fn default() -> Self {
        BTCOutputType::UNKNOWN
    }
}

impl From<i32> for BTCOutputType {
    fn from(i: i32) -> Self {
        match i {
            0 => BTCOutputType::UNKNOWN,
            1 => BTCOutputType::P2PKH,
            2 => BTCOutputType::P2SH,
            3 => BTCOutputType::P2WPKH,
            4 => BTCOutputType::P2WSH,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for BTCOutputType {
    fn from(s: &'a str) -> Self {
        match s {
            "UNKNOWN" => BTCOutputType::UNKNOWN,
            "P2PKH" => BTCOutputType::P2PKH,
            "P2SH" => BTCOutputType::P2SH,
            "P2WPKH" => BTCOutputType::P2WPKH,
            "P2WSH" => BTCOutputType::P2WSH,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BTCScriptConfig<'a> {
    pub config: mod_BTCScriptConfig::OneOfconfig<'a>,
}

impl<'a> MessageRead<'a> for BTCScriptConfig<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.config = mod_BTCScriptConfig::OneOfconfig::simple_type(r.read_enum(bytes)?),
                Ok(18) => msg.config = mod_BTCScriptConfig::OneOfconfig::multisig(r.read_message::<mod_BTCScriptConfig::Multisig>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BTCScriptConfig<'a> {
    fn get_size(&self) -> usize {
        0
        + match self.config {
            mod_BTCScriptConfig::OneOfconfig::simple_type(ref m) => 1 + sizeof_varint(*(m) as u64),
            mod_BTCScriptConfig::OneOfconfig::multisig(ref m) => 1 + sizeof_len((m).get_size()),
            mod_BTCScriptConfig::OneOfconfig::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.config {            mod_BTCScriptConfig::OneOfconfig::simple_type(ref m) => { w.write_with_tag(8, |w| w.write_enum(*m as i32))? },
            mod_BTCScriptConfig::OneOfconfig::multisig(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_BTCScriptConfig::OneOfconfig::None => {},
    }        Ok(())
    }
}

pub mod mod_BTCScriptConfig {

use alloc::vec::Vec;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Multisig<'a> {
    pub threshold: u32,
    pub xpubs: Vec<common::XPub<'a>>,
    pub our_xpub_index: u32,
}

impl<'a> MessageRead<'a> for Multisig<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.threshold = r.read_uint32(bytes)?,
                Ok(18) => msg.xpubs.push(r.read_message::<common::XPub>(bytes)?),
                Ok(24) => msg.our_xpub_index = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Multisig<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.threshold == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.threshold) as u64) }
        + self.xpubs.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.our_xpub_index == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.our_xpub_index) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.threshold != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.threshold))?; }
        for s in &self.xpubs { w.write_with_tag(18, |w| w.write_message(s))?; }
        if self.our_xpub_index != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.our_xpub_index))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SimpleType {
    P2WPKH_P2SH = 0,
    P2WPKH = 1,
}

impl Default for SimpleType {
    fn default() -> Self {
        SimpleType::P2WPKH_P2SH
    }
}

impl From<i32> for SimpleType {
    fn from(i: i32) -> Self {
        match i {
            0 => SimpleType::P2WPKH_P2SH,
            1 => SimpleType::P2WPKH,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for SimpleType {
    fn from(s: &'a str) -> Self {
        match s {
            "P2WPKH_P2SH" => SimpleType::P2WPKH_P2SH,
            "P2WPKH" => SimpleType::P2WPKH,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfconfig<'a> {
    simple_type(mod_BTCScriptConfig::SimpleType),
    multisig(mod_BTCScriptConfig::Multisig<'a>),
    None,
}

impl<'a> Default for OneOfconfig<'a> {
    fn default() -> Self {
        OneOfconfig::None
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BTCPubRequest<'a> {
    pub coin: BTCCoin,
    pub keypath: Vec<u32>,
    pub display: bool,
    pub output: mod_BTCPubRequest::OneOfoutput<'a>,
}

impl<'a> MessageRead<'a> for BTCPubRequest<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.coin = r.read_enum(bytes)?,
                Ok(18) => msg.keypath = r.read_packed(bytes, |r, bytes| Ok(r.read_uint32(bytes)?))?,
                Ok(40) => msg.display = r.read_bool(bytes)?,
                Ok(24) => msg.output = mod_BTCPubRequest::OneOfoutput::xpub_type(r.read_enum(bytes)?),
                Ok(34) => msg.output = mod_BTCPubRequest::OneOfoutput::script_config(r.read_message::<BTCScriptConfig>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BTCPubRequest<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.coin == btc::BTCCoin::BTC { 0 } else { 1 + sizeof_varint(*(&self.coin) as u64) }
        + if self.keypath.is_empty() { 0 } else { 1 + sizeof_len(self.keypath.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + if self.display == false { 0 } else { 1 + sizeof_varint(*(&self.display) as u64) }
        + match self.output {
            mod_BTCPubRequest::OneOfoutput::xpub_type(ref m) => 1 + sizeof_varint(*(m) as u64),
            mod_BTCPubRequest::OneOfoutput::script_config(ref m) => 1 + sizeof_len((m).get_size()),
            mod_BTCPubRequest::OneOfoutput::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.coin != btc::BTCCoin::BTC { w.write_with_tag(8, |w| w.write_enum(*&self.coin as i32))?; }
        w.write_packed_with_tag(18, &self.keypath, |w, m| w.write_uint32(*m), &|m| sizeof_varint(*(m) as u64))?;
        if self.display != false { w.write_with_tag(40, |w| w.write_bool(*&self.display))?; }
        match self.output {            mod_BTCPubRequest::OneOfoutput::xpub_type(ref m) => { w.write_with_tag(24, |w| w.write_enum(*m as i32))? },
            mod_BTCPubRequest::OneOfoutput::script_config(ref m) => { w.write_with_tag(34, |w| w.write_message(m))? },
            mod_BTCPubRequest::OneOfoutput::None => {},
    }        Ok(())
    }
}

pub mod mod_BTCPubRequest {

use alloc::vec::Vec;
use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum XPubType {
    TPUB = 0,
    XPUB = 1,
    YPUB = 2,
    ZPUB = 3,
    VPUB = 4,
    UPUB = 5,
    CAPITAL_VPUB = 6,
    CAPITAL_ZPUB = 7,
}

impl Default for XPubType {
    fn default() -> Self {
        XPubType::TPUB
    }
}

impl From<i32> for XPubType {
    fn from(i: i32) -> Self {
        match i {
            0 => XPubType::TPUB,
            1 => XPubType::XPUB,
            2 => XPubType::YPUB,
            3 => XPubType::ZPUB,
            4 => XPubType::VPUB,
            5 => XPubType::UPUB,
            6 => XPubType::CAPITAL_VPUB,
            7 => XPubType::CAPITAL_ZPUB,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for XPubType {
    fn from(s: &'a str) -> Self {
        match s {
            "TPUB" => XPubType::TPUB,
            "XPUB" => XPubType::XPUB,
            "YPUB" => XPubType::YPUB,
            "ZPUB" => XPubType::ZPUB,
            "VPUB" => XPubType::VPUB,
            "UPUB" => XPubType::UPUB,
            "CAPITAL_VPUB" => XPubType::CAPITAL_VPUB,
            "CAPITAL_ZPUB" => XPubType::CAPITAL_ZPUB,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfoutput<'a> {
    xpub_type(mod_BTCPubRequest::XPubType),
    script_config(BTCScriptConfig<'a>),
    None,
}

impl<'a> Default for OneOfoutput<'a> {
    fn default() -> Self {
        OneOfoutput::None
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BTCScriptConfigWithKeypath<'a> {
    pub script_config: Option<BTCScriptConfig<'a>>,
    pub keypath: Vec<u32>,
}

impl<'a> MessageRead<'a> for BTCScriptConfigWithKeypath<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(18) => msg.script_config = Some(r.read_message::<BTCScriptConfig>(bytes)?),
                Ok(26) => msg.keypath = r.read_packed(bytes, |r, bytes| Ok(r.read_uint32(bytes)?))?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BTCScriptConfigWithKeypath<'a> {
    fn get_size(&self) -> usize {
        0
        + self.script_config.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.keypath.is_empty() { 0 } else { 1 + sizeof_len(self.keypath.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.script_config { w.write_with_tag(18, |w| w.write_message(s))?; }
        w.write_packed_with_tag(26, &self.keypath, |w, m| w.write_uint32(*m), &|m| sizeof_varint(*(m) as u64))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BTCSignInitRequest<'a> {
    pub coin: BTCCoin,
    pub script_configs: Vec<BTCScriptConfigWithKeypath<'a>>,
    pub version: u32,
    pub num_inputs: u32,
    pub num_outputs: u32,
    pub locktime: u32,
}

impl<'a> MessageRead<'a> for BTCSignInitRequest<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.coin = r.read_enum(bytes)?,
                Ok(18) => msg.script_configs.push(r.read_message::<BTCScriptConfigWithKeypath>(bytes)?),
                Ok(32) => msg.version = r.read_uint32(bytes)?,
                Ok(40) => msg.num_inputs = r.read_uint32(bytes)?,
                Ok(48) => msg.num_outputs = r.read_uint32(bytes)?,
                Ok(56) => msg.locktime = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BTCSignInitRequest<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.coin == btc::BTCCoin::BTC { 0 } else { 1 + sizeof_varint(*(&self.coin) as u64) }
        + self.script_configs.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.version == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.version) as u64) }
        + if self.num_inputs == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.num_inputs) as u64) }
        + if self.num_outputs == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.num_outputs) as u64) }
        + if self.locktime == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.locktime) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.coin != btc::BTCCoin::BTC { w.write_with_tag(8, |w| w.write_enum(*&self.coin as i32))?; }
        for s in &self.script_configs { w.write_with_tag(18, |w| w.write_message(s))?; }
        if self.version != 0u32 { w.write_with_tag(32, |w| w.write_uint32(*&self.version))?; }
        if self.num_inputs != 0u32 { w.write_with_tag(40, |w| w.write_uint32(*&self.num_inputs))?; }
        if self.num_outputs != 0u32 { w.write_with_tag(48, |w| w.write_uint32(*&self.num_outputs))?; }
        if self.locktime != 0u32 { w.write_with_tag(56, |w| w.write_uint32(*&self.locktime))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BTCSignNextResponse<'a> {
    pub type_pb: mod_BTCSignNextResponse::Type,
    pub index: u32,
    pub has_signature: bool,
    pub signature: Cow<'a, [u8]>,
    pub prev_index: u32,
}

impl<'a> MessageRead<'a> for BTCSignNextResponse<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = r.read_enum(bytes)?,
                Ok(16) => msg.index = r.read_uint32(bytes)?,
                Ok(24) => msg.has_signature = r.read_bool(bytes)?,
                Ok(34) => msg.signature = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(40) => msg.prev_index = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BTCSignNextResponse<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.type_pb == btc::mod_BTCSignNextResponse::Type::INPUT { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
        + if self.index == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.index) as u64) }
        + if self.has_signature == false { 0 } else { 1 + sizeof_varint(*(&self.has_signature) as u64) }
        + if self.signature == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.signature).len()) }
        + if self.prev_index == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.prev_index) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.type_pb != btc::mod_BTCSignNextResponse::Type::INPUT { w.write_with_tag(8, |w| w.write_enum(*&self.type_pb as i32))?; }
        if self.index != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.index))?; }
        if self.has_signature != false { w.write_with_tag(24, |w| w.write_bool(*&self.has_signature))?; }
        if self.signature != Cow::Borrowed(b"") { w.write_with_tag(34, |w| w.write_bytes(&**&self.signature))?; }
        if self.prev_index != 0u32 { w.write_with_tag(40, |w| w.write_uint32(*&self.prev_index))?; }
        Ok(())
    }
}

pub mod mod_BTCSignNextResponse {

use alloc::vec::Vec;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    INPUT = 0,
    OUTPUT = 1,
    DONE = 2,
    PREVTX_INIT = 3,
    PREVTX_INPUT = 4,
    PREVTX_OUTPUT = 5,
}

impl Default for Type {
    fn default() -> Self {
        Type::INPUT
    }
}

impl From<i32> for Type {
    fn from(i: i32) -> Self {
        match i {
            0 => Type::INPUT,
            1 => Type::OUTPUT,
            2 => Type::DONE,
            3 => Type::PREVTX_INIT,
            4 => Type::PREVTX_INPUT,
            5 => Type::PREVTX_OUTPUT,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Type {
    fn from(s: &'a str) -> Self {
        match s {
            "INPUT" => Type::INPUT,
            "OUTPUT" => Type::OUTPUT,
            "DONE" => Type::DONE,
            "PREVTX_INIT" => Type::PREVTX_INIT,
            "PREVTX_INPUT" => Type::PREVTX_INPUT,
            "PREVTX_OUTPUT" => Type::PREVTX_OUTPUT,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BTCSignInputRequest<'a> {
    pub prevOutHash: Cow<'a, [u8]>,
    pub prevOutIndex: u32,
    pub prevOutValue: u64,
    pub sequence: u32,
    pub keypath: Vec<u32>,
    pub script_config_index: u32,
}

impl<'a> MessageRead<'a> for BTCSignInputRequest<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.prevOutHash = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.prevOutIndex = r.read_uint32(bytes)?,
                Ok(24) => msg.prevOutValue = r.read_uint64(bytes)?,
                Ok(32) => msg.sequence = r.read_uint32(bytes)?,
                Ok(50) => msg.keypath = r.read_packed(bytes, |r, bytes| Ok(r.read_uint32(bytes)?))?,
                Ok(56) => msg.script_config_index = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BTCSignInputRequest<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.prevOutHash == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.prevOutHash).len()) }
        + if self.prevOutIndex == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.prevOutIndex) as u64) }
        + if self.prevOutValue == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.prevOutValue) as u64) }
        + if self.sequence == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.sequence) as u64) }
        + if self.keypath.is_empty() { 0 } else { 1 + sizeof_len(self.keypath.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + if self.script_config_index == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.script_config_index) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.prevOutHash != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.prevOutHash))?; }
        if self.prevOutIndex != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.prevOutIndex))?; }
        if self.prevOutValue != 0u64 { w.write_with_tag(24, |w| w.write_uint64(*&self.prevOutValue))?; }
        if self.sequence != 0u32 { w.write_with_tag(32, |w| w.write_uint32(*&self.sequence))?; }
        w.write_packed_with_tag(50, &self.keypath, |w, m| w.write_uint32(*m), &|m| sizeof_varint(*(m) as u64))?;
        if self.script_config_index != 0u32 { w.write_with_tag(56, |w| w.write_uint32(*&self.script_config_index))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BTCSignOutputRequest<'a> {
    pub ours: bool,
    pub type_pb: BTCOutputType,
    pub value: u64,
    pub hash: Cow<'a, [u8]>,
    pub keypath: Vec<u32>,
    pub script_config_index: u32,
}

impl<'a> MessageRead<'a> for BTCSignOutputRequest<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.ours = r.read_bool(bytes)?,
                Ok(16) => msg.type_pb = r.read_enum(bytes)?,
                Ok(24) => msg.value = r.read_uint64(bytes)?,
                Ok(34) => msg.hash = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.keypath = r.read_packed(bytes, |r, bytes| Ok(r.read_uint32(bytes)?))?,
                Ok(48) => msg.script_config_index = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BTCSignOutputRequest<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.ours == false { 0 } else { 1 + sizeof_varint(*(&self.ours) as u64) }
        + if self.type_pb == btc::BTCOutputType::UNKNOWN { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
        + if self.value == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.value) as u64) }
        + if self.hash == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.hash).len()) }
        + if self.keypath.is_empty() { 0 } else { 1 + sizeof_len(self.keypath.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + if self.script_config_index == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.script_config_index) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.ours != false { w.write_with_tag(8, |w| w.write_bool(*&self.ours))?; }
        if self.type_pb != btc::BTCOutputType::UNKNOWN { w.write_with_tag(16, |w| w.write_enum(*&self.type_pb as i32))?; }
        if self.value != 0u64 { w.write_with_tag(24, |w| w.write_uint64(*&self.value))?; }
        if self.hash != Cow::Borrowed(b"") { w.write_with_tag(34, |w| w.write_bytes(&**&self.hash))?; }
        w.write_packed_with_tag(42, &self.keypath, |w, m| w.write_uint32(*m), &|m| sizeof_varint(*(m) as u64))?;
        if self.script_config_index != 0u32 { w.write_with_tag(48, |w| w.write_uint32(*&self.script_config_index))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BTCScriptConfigRegistration<'a> {
    pub coin: BTCCoin,
    pub script_config: Option<BTCScriptConfig<'a>>,
    pub keypath: Vec<u32>,
}

impl<'a> MessageRead<'a> for BTCScriptConfigRegistration<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.coin = r.read_enum(bytes)?,
                Ok(18) => msg.script_config = Some(r.read_message::<BTCScriptConfig>(bytes)?),
                Ok(26) => msg.keypath = r.read_packed(bytes, |r, bytes| Ok(r.read_uint32(bytes)?))?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BTCScriptConfigRegistration<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.coin == btc::BTCCoin::BTC { 0 } else { 1 + sizeof_varint(*(&self.coin) as u64) }
        + self.script_config.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.keypath.is_empty() { 0 } else { 1 + sizeof_len(self.keypath.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.coin != btc::BTCCoin::BTC { w.write_with_tag(8, |w| w.write_enum(*&self.coin as i32))?; }
        if let Some(ref s) = self.script_config { w.write_with_tag(18, |w| w.write_message(s))?; }
        w.write_packed_with_tag(26, &self.keypath, |w, m| w.write_uint32(*m), &|m| sizeof_varint(*(m) as u64))?;
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BTCSuccess { }

impl<'a> MessageRead<'a> for BTCSuccess {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for BTCSuccess { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BTCIsScriptConfigRegisteredRequest<'a> {
    pub registration: Option<BTCScriptConfigRegistration<'a>>,
}

impl<'a> MessageRead<'a> for BTCIsScriptConfigRegisteredRequest<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.registration = Some(r.read_message::<BTCScriptConfigRegistration>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BTCIsScriptConfigRegisteredRequest<'a> {
    fn get_size(&self) -> usize {
        0
        + self.registration.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.registration { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BTCIsScriptConfigRegisteredResponse {
    pub is_registered: bool,
}

impl<'a> MessageRead<'a> for BTCIsScriptConfigRegisteredResponse {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.is_registered = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for BTCIsScriptConfigRegisteredResponse {
    fn get_size(&self) -> usize {
        0
        + if self.is_registered == false { 0 } else { 1 + sizeof_varint(*(&self.is_registered) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.is_registered != false { w.write_with_tag(8, |w| w.write_bool(*&self.is_registered))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BTCRegisterScriptConfigRequest<'a> {
    pub registration: Option<BTCScriptConfigRegistration<'a>>,
    pub name: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for BTCRegisterScriptConfigRequest<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.registration = Some(r.read_message::<BTCScriptConfigRegistration>(bytes)?),
                Ok(18) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BTCRegisterScriptConfigRequest<'a> {
    fn get_size(&self) -> usize {
        0
        + self.registration.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.name == "" { 0 } else { 1 + sizeof_len((&self.name).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.registration { w.write_with_tag(10, |w| w.write_message(s))?; }
        if self.name != "" { w.write_with_tag(18, |w| w.write_string(&**&self.name))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BTCPrevTxInitRequest {
    pub version: u32,
    pub num_inputs: u32,
    pub num_outputs: u32,
    pub locktime: u32,
}

impl<'a> MessageRead<'a> for BTCPrevTxInitRequest {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.version = r.read_uint32(bytes)?,
                Ok(16) => msg.num_inputs = r.read_uint32(bytes)?,
                Ok(24) => msg.num_outputs = r.read_uint32(bytes)?,
                Ok(32) => msg.locktime = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for BTCPrevTxInitRequest {
    fn get_size(&self) -> usize {
        0
        + if self.version == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.version) as u64) }
        + if self.num_inputs == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.num_inputs) as u64) }
        + if self.num_outputs == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.num_outputs) as u64) }
        + if self.locktime == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.locktime) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.version != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.version))?; }
        if self.num_inputs != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.num_inputs))?; }
        if self.num_outputs != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.num_outputs))?; }
        if self.locktime != 0u32 { w.write_with_tag(32, |w| w.write_uint32(*&self.locktime))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BTCPrevTxInputRequest<'a> {
    pub prev_out_hash: Cow<'a, [u8]>,
    pub prev_out_index: u32,
    pub signature_script: Cow<'a, [u8]>,
    pub sequence: u32,
}

impl<'a> MessageRead<'a> for BTCPrevTxInputRequest<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.prev_out_hash = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.prev_out_index = r.read_uint32(bytes)?,
                Ok(26) => msg.signature_script = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(32) => msg.sequence = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BTCPrevTxInputRequest<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.prev_out_hash == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.prev_out_hash).len()) }
        + if self.prev_out_index == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.prev_out_index) as u64) }
        + if self.signature_script == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.signature_script).len()) }
        + if self.sequence == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.sequence) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.prev_out_hash != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.prev_out_hash))?; }
        if self.prev_out_index != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.prev_out_index))?; }
        if self.signature_script != Cow::Borrowed(b"") { w.write_with_tag(26, |w| w.write_bytes(&**&self.signature_script))?; }
        if self.sequence != 0u32 { w.write_with_tag(32, |w| w.write_uint32(*&self.sequence))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BTCPrevTxOutputRequest<'a> {
    pub value: u64,
    pub pubkey_script: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for BTCPrevTxOutputRequest<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.value = r.read_uint64(bytes)?,
                Ok(18) => msg.pubkey_script = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BTCPrevTxOutputRequest<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.value == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.value) as u64) }
        + if self.pubkey_script == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.pubkey_script).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.value))?; }
        if self.pubkey_script != Cow::Borrowed(b"") { w.write_with_tag(18, |w| w.write_bytes(&**&self.pubkey_script))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BTCRequest<'a> {
    pub request: mod_BTCRequest::OneOfrequest<'a>,
}

impl<'a> MessageRead<'a> for BTCRequest<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.request = mod_BTCRequest::OneOfrequest::is_script_config_registered(r.read_message::<BTCIsScriptConfigRegisteredRequest>(bytes)?),
                Ok(18) => msg.request = mod_BTCRequest::OneOfrequest::register_script_config(r.read_message::<BTCRegisterScriptConfigRequest>(bytes)?),
                Ok(26) => msg.request = mod_BTCRequest::OneOfrequest::prevtx_init(r.read_message::<BTCPrevTxInitRequest>(bytes)?),
                Ok(34) => msg.request = mod_BTCRequest::OneOfrequest::prevtx_input(r.read_message::<BTCPrevTxInputRequest>(bytes)?),
                Ok(42) => msg.request = mod_BTCRequest::OneOfrequest::prevtx_output(r.read_message::<BTCPrevTxOutputRequest>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BTCRequest<'a> {
    fn get_size(&self) -> usize {
        0
        + match self.request {
            mod_BTCRequest::OneOfrequest::is_script_config_registered(ref m) => 1 + sizeof_len((m).get_size()),
            mod_BTCRequest::OneOfrequest::register_script_config(ref m) => 1 + sizeof_len((m).get_size()),
            mod_BTCRequest::OneOfrequest::prevtx_init(ref m) => 1 + sizeof_len((m).get_size()),
            mod_BTCRequest::OneOfrequest::prevtx_input(ref m) => 1 + sizeof_len((m).get_size()),
            mod_BTCRequest::OneOfrequest::prevtx_output(ref m) => 1 + sizeof_len((m).get_size()),
            mod_BTCRequest::OneOfrequest::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.request {            mod_BTCRequest::OneOfrequest::is_script_config_registered(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            mod_BTCRequest::OneOfrequest::register_script_config(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_BTCRequest::OneOfrequest::prevtx_init(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            mod_BTCRequest::OneOfrequest::prevtx_input(ref m) => { w.write_with_tag(34, |w| w.write_message(m))? },
            mod_BTCRequest::OneOfrequest::prevtx_output(ref m) => { w.write_with_tag(42, |w| w.write_message(m))? },
            mod_BTCRequest::OneOfrequest::None => {},
    }        Ok(())
    }
}

pub mod mod_BTCRequest {

use alloc::vec::Vec;
use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfrequest<'a> {
    is_script_config_registered(BTCIsScriptConfigRegisteredRequest<'a>),
    register_script_config(BTCRegisterScriptConfigRequest<'a>),
    prevtx_init(BTCPrevTxInitRequest),
    prevtx_input(BTCPrevTxInputRequest<'a>),
    prevtx_output(BTCPrevTxOutputRequest<'a>),
    None,
}

impl<'a> Default for OneOfrequest<'a> {
    fn default() -> Self {
        OneOfrequest::None
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BTCResponse<'a> {
    pub response: mod_BTCResponse::OneOfresponse<'a>,
}

impl<'a> MessageRead<'a> for BTCResponse<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.response = mod_BTCResponse::OneOfresponse::success(r.read_message::<BTCSuccess>(bytes)?),
                Ok(18) => msg.response = mod_BTCResponse::OneOfresponse::is_script_config_registered(r.read_message::<BTCIsScriptConfigRegisteredResponse>(bytes)?),
                Ok(26) => msg.response = mod_BTCResponse::OneOfresponse::sign_next(r.read_message::<BTCSignNextResponse>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BTCResponse<'a> {
    fn get_size(&self) -> usize {
        0
        + match self.response {
            mod_BTCResponse::OneOfresponse::success(ref m) => 1 + sizeof_len((m).get_size()),
            mod_BTCResponse::OneOfresponse::is_script_config_registered(ref m) => 1 + sizeof_len((m).get_size()),
            mod_BTCResponse::OneOfresponse::sign_next(ref m) => 1 + sizeof_len((m).get_size()),
            mod_BTCResponse::OneOfresponse::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.response {            mod_BTCResponse::OneOfresponse::success(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            mod_BTCResponse::OneOfresponse::is_script_config_registered(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_BTCResponse::OneOfresponse::sign_next(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            mod_BTCResponse::OneOfresponse::None => {},
    }        Ok(())
    }
}

pub mod mod_BTCResponse {

use alloc::vec::Vec;
use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfresponse<'a> {
    success(BTCSuccess),
    is_script_config_registered(BTCIsScriptConfigRegisteredResponse),
    sign_next(BTCSignNextResponse<'a>),
    None,
}

impl<'a> Default for OneOfresponse<'a> {
    fn default() -> Self {
        OneOfresponse::None
    }
}

}


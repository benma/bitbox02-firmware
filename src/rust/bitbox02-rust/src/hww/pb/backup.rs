// Automatically generated rust module for 'backup.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec::Vec;
use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BackupMode {
    PLAINTEXT = 0,
}

impl Default for BackupMode {
    fn default() -> Self {
        BackupMode::PLAINTEXT
    }
}

impl From<i32> for BackupMode {
    fn from(i: i32) -> Self {
        match i {
            0 => BackupMode::PLAINTEXT,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for BackupMode {
    fn from(s: &'a str) -> Self {
        match s {
            "PLAINTEXT" => BackupMode::PLAINTEXT,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BackupMetaData {
    pub timestamp: u32,
    pub name: String,
    pub mode: BackupMode,
}

impl<'a> MessageRead<'a> for BackupMetaData {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.timestamp = r.read_uint32(bytes)?,
                Ok(18) => msg.name = r.read_string(bytes)?.to_owned(),
                Ok(24) => msg.mode = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for BackupMetaData {
    fn get_size(&self) -> usize {
        0
        + if self.timestamp == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.timestamp) as u64) }
        + if self.name == String::default() { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + if self.mode == backup::BackupMode::PLAINTEXT { 0 } else { 1 + sizeof_varint(*(&self.mode) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.timestamp != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.timestamp))?; }
        if self.name != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.name))?; }
        if self.mode != backup::BackupMode::PLAINTEXT { w.write_with_tag(24, |w| w.write_enum(*&self.mode as i32))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BackupData {
    pub seed_length: u32,
    pub seed: Vec<u8>,
    pub birthdate: u32,
    pub generator: String,
}

impl<'a> MessageRead<'a> for BackupData {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.seed_length = r.read_uint32(bytes)?,
                Ok(18) => msg.seed = r.read_bytes(bytes)?.to_owned(),
                Ok(24) => msg.birthdate = r.read_uint32(bytes)?,
                Ok(34) => msg.generator = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for BackupData {
    fn get_size(&self) -> usize {
        0
        + if self.seed_length == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.seed_length) as u64) }
        + if self.seed == vec![] { 0 } else { 1 + sizeof_len((&self.seed).len()) }
        + if self.birthdate == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.birthdate) as u64) }
        + if self.generator == String::default() { 0 } else { 1 + sizeof_len((&self.generator).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.seed_length != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.seed_length))?; }
        if self.seed != vec![] { w.write_with_tag(18, |w| w.write_bytes(&**&self.seed))?; }
        if self.birthdate != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.birthdate))?; }
        if self.generator != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.generator))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BackupContent {
    pub checksum: Vec<u8>,
    pub metadata: Option<BackupMetaData>,
    pub length: u32,
    pub data: Vec<u8>,
}

impl<'a> MessageRead<'a> for BackupContent {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.checksum = r.read_bytes(bytes)?.to_owned(),
                Ok(18) => msg.metadata = Some(r.read_message::<BackupMetaData>(bytes)?),
                Ok(24) => msg.length = r.read_uint32(bytes)?,
                Ok(34) => msg.data = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for BackupContent {
    fn get_size(&self) -> usize {
        0
        + if self.checksum == vec![] { 0 } else { 1 + sizeof_len((&self.checksum).len()) }
        + self.metadata.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.length == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.length) as u64) }
        + if self.data == vec![] { 0 } else { 1 + sizeof_len((&self.data).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.checksum != vec![] { w.write_with_tag(10, |w| w.write_bytes(&**&self.checksum))?; }
        if let Some(ref s) = self.metadata { w.write_with_tag(18, |w| w.write_message(s))?; }
        if self.length != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.length))?; }
        if self.data != vec![] { w.write_with_tag(34, |w| w.write_bytes(&**&self.data))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BackupV1 {
    pub content: Option<BackupContent>,
}

impl<'a> MessageRead<'a> for BackupV1 {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.content = Some(r.read_message::<BackupContent>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for BackupV1 {
    fn get_size(&self) -> usize {
        0
        + self.content.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.content { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Backup {
    pub backup_version: mod_Backup::OneOfbackup_version,
}

impl<'a> MessageRead<'a> for Backup {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.backup_version = mod_Backup::OneOfbackup_version::backup_v1(r.read_message::<BackupV1>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Backup {
    fn get_size(&self) -> usize {
        0
        + match self.backup_version {
            mod_Backup::OneOfbackup_version::backup_v1(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Backup::OneOfbackup_version::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.backup_version {            mod_Backup::OneOfbackup_version::backup_v1(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            mod_Backup::OneOfbackup_version::None => {},
    }        Ok(())
    }
}

pub mod mod_Backup {

use alloc::vec::Vec;
use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfbackup_version {
    backup_v1(BackupV1),
    None,
}

impl Default for OneOfbackup_version {
    fn default() -> Self {
        OneOfbackup_version::None
    }
}

}

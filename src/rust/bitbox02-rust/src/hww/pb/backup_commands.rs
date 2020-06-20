// Automatically generated rust module for 'backup_commands.proto' file

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
use alloc::string::String;
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CheckBackupRequest {
    pub silent: bool,
}

impl<'a> MessageRead<'a> for CheckBackupRequest {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.silent = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for CheckBackupRequest {
    fn get_size(&self) -> usize {
        0
        + if self.silent == false { 0 } else { 1 + sizeof_varint(*(&self.silent) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.silent != false { w.write_with_tag(8, |w| w.write_bool(*&self.silent))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CheckBackupResponse {
    pub id: String,
}

impl<'a> MessageRead<'a> for CheckBackupResponse {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for CheckBackupResponse {
    fn get_size(&self) -> usize {
        0
        + if self.id == String::default() { 0 } else { 1 + sizeof_len((&self.id).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CreateBackupRequest {
    pub timestamp: u32,
    pub timezone_offset: i32,
}

impl<'a> MessageRead<'a> for CreateBackupRequest {
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

impl MessageWrite for CreateBackupRequest {
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
pub struct ListBackupsRequest { }

impl<'a> MessageRead<'a> for ListBackupsRequest {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for ListBackupsRequest { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BackupInfo {
    pub id: String,
    pub timestamp: u32,
    pub name: String,
}

impl<'a> MessageRead<'a> for BackupInfo {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes)?.to_owned(),
                Ok(16) => msg.timestamp = r.read_uint32(bytes)?,
                Ok(34) => msg.name = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for BackupInfo {
    fn get_size(&self) -> usize {
        0
        + if self.id == String::default() { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.timestamp == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.timestamp) as u64) }
        + if self.name == String::default() { 0 } else { 1 + sizeof_len((&self.name).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if self.timestamp != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.timestamp))?; }
        if self.name != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.name))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ListBackupsResponse {
    pub info: Vec<BackupInfo>,
}

impl<'a> MessageRead<'a> for ListBackupsResponse {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.info.push(r.read_message::<BackupInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ListBackupsResponse {
    fn get_size(&self) -> usize {
        0
        + self.info.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.info { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RestoreBackupRequest {
    pub id: String,
    pub timestamp: u32,
    pub timezone_offset: i32,
}

impl<'a> MessageRead<'a> for RestoreBackupRequest {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes)?.to_owned(),
                Ok(16) => msg.timestamp = r.read_uint32(bytes)?,
                Ok(24) => msg.timezone_offset = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RestoreBackupRequest {
    fn get_size(&self) -> usize {
        0
        + if self.id == String::default() { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.timestamp == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.timestamp) as u64) }
        + if self.timezone_offset == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.timezone_offset) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if self.timestamp != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.timestamp))?; }
        if self.timezone_offset != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.timezone_offset))?; }
        Ok(())
    }
}

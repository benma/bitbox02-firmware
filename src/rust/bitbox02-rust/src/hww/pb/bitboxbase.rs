// Automatically generated rust module for 'bitboxbase.proto' file

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

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BitBoxBaseHeartbeatRequest {
    pub state_code: mod_BitBoxBaseHeartbeatRequest::StateCode,
    pub description_code: mod_BitBoxBaseHeartbeatRequest::DescriptionCode,
}

impl<'a> MessageRead<'a> for BitBoxBaseHeartbeatRequest {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.state_code = r.read_enum(bytes)?,
                Ok(16) => msg.description_code = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for BitBoxBaseHeartbeatRequest {
    fn get_size(&self) -> usize {
        0
        + if self.state_code == bitboxbase::mod_BitBoxBaseHeartbeatRequest::StateCode::IDLE { 0 } else { 1 + sizeof_varint(*(&self.state_code) as u64) }
        + if self.description_code == bitboxbase::mod_BitBoxBaseHeartbeatRequest::DescriptionCode::EMPTY { 0 } else { 1 + sizeof_varint(*(&self.description_code) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.state_code != bitboxbase::mod_BitBoxBaseHeartbeatRequest::StateCode::IDLE { w.write_with_tag(8, |w| w.write_enum(*&self.state_code as i32))?; }
        if self.description_code != bitboxbase::mod_BitBoxBaseHeartbeatRequest::DescriptionCode::EMPTY { w.write_with_tag(16, |w| w.write_enum(*&self.description_code as i32))?; }
        Ok(())
    }
}

pub mod mod_BitBoxBaseHeartbeatRequest {

use alloc::vec::Vec;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StateCode {
    IDLE = 0,
    WORKING = 1,
    WARNING = 2,
    ERROR = 3,
}

impl Default for StateCode {
    fn default() -> Self {
        StateCode::IDLE
    }
}

impl From<i32> for StateCode {
    fn from(i: i32) -> Self {
        match i {
            0 => StateCode::IDLE,
            1 => StateCode::WORKING,
            2 => StateCode::WARNING,
            3 => StateCode::ERROR,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for StateCode {
    fn from(s: &'a str) -> Self {
        match s {
            "IDLE" => StateCode::IDLE,
            "WORKING" => StateCode::WORKING,
            "WARNING" => StateCode::WARNING,
            "ERROR" => StateCode::ERROR,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DescriptionCode {
    EMPTY = 0,
    INITIAL_BLOCK_SYNC = 1,
    DOWNLOAD_UPDATE = 2,
    OUT_OF_DISK_SPACE = 3,
    REDIS_ERROR = 4,
    REBOOT = 5,
    SHUTDOWN = 6,
    UPDATE_FAILED = 7,
    NO_NETWORK_CONNECTION = 8,
}

impl Default for DescriptionCode {
    fn default() -> Self {
        DescriptionCode::EMPTY
    }
}

impl From<i32> for DescriptionCode {
    fn from(i: i32) -> Self {
        match i {
            0 => DescriptionCode::EMPTY,
            1 => DescriptionCode::INITIAL_BLOCK_SYNC,
            2 => DescriptionCode::DOWNLOAD_UPDATE,
            3 => DescriptionCode::OUT_OF_DISK_SPACE,
            4 => DescriptionCode::REDIS_ERROR,
            5 => DescriptionCode::REBOOT,
            6 => DescriptionCode::SHUTDOWN,
            7 => DescriptionCode::UPDATE_FAILED,
            8 => DescriptionCode::NO_NETWORK_CONNECTION,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for DescriptionCode {
    fn from(s: &'a str) -> Self {
        match s {
            "EMPTY" => DescriptionCode::EMPTY,
            "INITIAL_BLOCK_SYNC" => DescriptionCode::INITIAL_BLOCK_SYNC,
            "DOWNLOAD_UPDATE" => DescriptionCode::DOWNLOAD_UPDATE,
            "OUT_OF_DISK_SPACE" => DescriptionCode::OUT_OF_DISK_SPACE,
            "REDIS_ERROR" => DescriptionCode::REDIS_ERROR,
            "REBOOT" => DescriptionCode::REBOOT,
            "SHUTDOWN" => DescriptionCode::SHUTDOWN,
            "UPDATE_FAILED" => DescriptionCode::UPDATE_FAILED,
            "NO_NETWORK_CONNECTION" => DescriptionCode::NO_NETWORK_CONNECTION,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BitBoxBaseConfirmPairingRequest {
    pub msg: Vec<u8>,
}

impl<'a> MessageRead<'a> for BitBoxBaseConfirmPairingRequest {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.msg = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for BitBoxBaseConfirmPairingRequest {
    fn get_size(&self) -> usize {
        0
        + if self.msg == vec![] { 0 } else { 1 + sizeof_len((&self.msg).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.msg != vec![] { w.write_with_tag(10, |w| w.write_bytes(&**&self.msg))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BitBoxBaseSetConfigRequest {
    pub status_led_mode: mod_BitBoxBaseSetConfigRequest::StatusLedMode,
    pub status_screen_mode: mod_BitBoxBaseSetConfigRequest::StatusScreenMode,
    pub hostname: String,
    pub ip_option: mod_BitBoxBaseSetConfigRequest::OneOfip_option,
}

impl<'a> MessageRead<'a> for BitBoxBaseSetConfigRequest {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.status_led_mode = r.read_enum(bytes)?,
                Ok(16) => msg.status_screen_mode = r.read_enum(bytes)?,
                Ok(34) => msg.hostname = r.read_string(bytes)?.to_owned(),
                Ok(26) => msg.ip_option = mod_BitBoxBaseSetConfigRequest::OneOfip_option::ip(r.read_bytes(bytes)?.to_owned()),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for BitBoxBaseSetConfigRequest {
    fn get_size(&self) -> usize {
        0
        + if self.status_led_mode == bitboxbase::mod_BitBoxBaseSetConfigRequest::StatusLedMode::LED_ALWAYS { 0 } else { 1 + sizeof_varint(*(&self.status_led_mode) as u64) }
        + if self.status_screen_mode == bitboxbase::mod_BitBoxBaseSetConfigRequest::StatusScreenMode::SCREEN_ALWAYS { 0 } else { 1 + sizeof_varint(*(&self.status_screen_mode) as u64) }
        + if self.hostname == String::default() { 0 } else { 1 + sizeof_len((&self.hostname).len()) }
        + match self.ip_option {
            mod_BitBoxBaseSetConfigRequest::OneOfip_option::ip(ref m) => 1 + sizeof_len((m).len()),
            mod_BitBoxBaseSetConfigRequest::OneOfip_option::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.status_led_mode != bitboxbase::mod_BitBoxBaseSetConfigRequest::StatusLedMode::LED_ALWAYS { w.write_with_tag(8, |w| w.write_enum(*&self.status_led_mode as i32))?; }
        if self.status_screen_mode != bitboxbase::mod_BitBoxBaseSetConfigRequest::StatusScreenMode::SCREEN_ALWAYS { w.write_with_tag(16, |w| w.write_enum(*&self.status_screen_mode as i32))?; }
        if self.hostname != String::default() { w.write_with_tag(34, |w| w.write_string(&**&self.hostname))?; }
        match self.ip_option {            mod_BitBoxBaseSetConfigRequest::OneOfip_option::ip(ref m) => { w.write_with_tag(26, |w| w.write_bytes(&**m))? },
            mod_BitBoxBaseSetConfigRequest::OneOfip_option::None => {},
    }        Ok(())
    }
}

pub mod mod_BitBoxBaseSetConfigRequest {

use alloc::vec::Vec;
use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StatusLedMode {
    LED_ALWAYS = 0,
    LED_ON_WORKING = 1,
    LED_ON_WARNING = 2,
    LED_ON_ERROR = 3,
}

impl Default for StatusLedMode {
    fn default() -> Self {
        StatusLedMode::LED_ALWAYS
    }
}

impl From<i32> for StatusLedMode {
    fn from(i: i32) -> Self {
        match i {
            0 => StatusLedMode::LED_ALWAYS,
            1 => StatusLedMode::LED_ON_WORKING,
            2 => StatusLedMode::LED_ON_WARNING,
            3 => StatusLedMode::LED_ON_ERROR,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for StatusLedMode {
    fn from(s: &'a str) -> Self {
        match s {
            "LED_ALWAYS" => StatusLedMode::LED_ALWAYS,
            "LED_ON_WORKING" => StatusLedMode::LED_ON_WORKING,
            "LED_ON_WARNING" => StatusLedMode::LED_ON_WARNING,
            "LED_ON_ERROR" => StatusLedMode::LED_ON_ERROR,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StatusScreenMode {
    SCREEN_ALWAYS = 0,
    SCREEN_ON_WORKING = 1,
    SCREEN_ON_WARNING = 2,
    SCREEN_ON_ERROR = 3,
}

impl Default for StatusScreenMode {
    fn default() -> Self {
        StatusScreenMode::SCREEN_ALWAYS
    }
}

impl From<i32> for StatusScreenMode {
    fn from(i: i32) -> Self {
        match i {
            0 => StatusScreenMode::SCREEN_ALWAYS,
            1 => StatusScreenMode::SCREEN_ON_WORKING,
            2 => StatusScreenMode::SCREEN_ON_WARNING,
            3 => StatusScreenMode::SCREEN_ON_ERROR,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for StatusScreenMode {
    fn from(s: &'a str) -> Self {
        match s {
            "SCREEN_ALWAYS" => StatusScreenMode::SCREEN_ALWAYS,
            "SCREEN_ON_WORKING" => StatusScreenMode::SCREEN_ON_WORKING,
            "SCREEN_ON_WARNING" => StatusScreenMode::SCREEN_ON_WARNING,
            "SCREEN_ON_ERROR" => StatusScreenMode::SCREEN_ON_ERROR,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfip_option {
    ip(Vec<u8>),
    None,
}

impl Default for OneOfip_option {
    fn default() -> Self {
        OneOfip_option::None
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BitBoxBaseDisplayStatusRequest {
    pub duration: u32,
}

impl<'a> MessageRead<'a> for BitBoxBaseDisplayStatusRequest {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.duration = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for BitBoxBaseDisplayStatusRequest {
    fn get_size(&self) -> usize {
        0
        + if self.duration == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.duration) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.duration != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.duration))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct BitBoxBaseRequest {
    pub request: mod_BitBoxBaseRequest::OneOfrequest,
}

impl<'a> MessageRead<'a> for BitBoxBaseRequest {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.request = mod_BitBoxBaseRequest::OneOfrequest::heartbeat(r.read_message::<BitBoxBaseHeartbeatRequest>(bytes)?),
                Ok(18) => msg.request = mod_BitBoxBaseRequest::OneOfrequest::set_config(r.read_message::<BitBoxBaseSetConfigRequest>(bytes)?),
                Ok(26) => msg.request = mod_BitBoxBaseRequest::OneOfrequest::confirm_pairing(r.read_message::<BitBoxBaseConfirmPairingRequest>(bytes)?),
                Ok(34) => msg.request = mod_BitBoxBaseRequest::OneOfrequest::display_status(r.read_message::<BitBoxBaseDisplayStatusRequest>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for BitBoxBaseRequest {
    fn get_size(&self) -> usize {
        0
        + match self.request {
            mod_BitBoxBaseRequest::OneOfrequest::heartbeat(ref m) => 1 + sizeof_len((m).get_size()),
            mod_BitBoxBaseRequest::OneOfrequest::set_config(ref m) => 1 + sizeof_len((m).get_size()),
            mod_BitBoxBaseRequest::OneOfrequest::confirm_pairing(ref m) => 1 + sizeof_len((m).get_size()),
            mod_BitBoxBaseRequest::OneOfrequest::display_status(ref m) => 1 + sizeof_len((m).get_size()),
            mod_BitBoxBaseRequest::OneOfrequest::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.request {            mod_BitBoxBaseRequest::OneOfrequest::heartbeat(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            mod_BitBoxBaseRequest::OneOfrequest::set_config(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_BitBoxBaseRequest::OneOfrequest::confirm_pairing(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            mod_BitBoxBaseRequest::OneOfrequest::display_status(ref m) => { w.write_with_tag(34, |w| w.write_message(m))? },
            mod_BitBoxBaseRequest::OneOfrequest::None => {},
    }        Ok(())
    }
}

pub mod mod_BitBoxBaseRequest {

use alloc::vec::Vec;
use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfrequest {
    heartbeat(BitBoxBaseHeartbeatRequest),
    set_config(BitBoxBaseSetConfigRequest),
    confirm_pairing(BitBoxBaseConfirmPairingRequest),
    display_status(BitBoxBaseDisplayStatusRequest),
    None,
}

impl Default for OneOfrequest {
    fn default() -> Self {
        OneOfrequest::None
    }
}

}

// Automatically generated rust module for 'hww.proto' file

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
pub struct Error<'a> {
    pub code: i32,
    pub message: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Error<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.code = r.read_int32(bytes)?,
                Ok(18) => msg.message = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Error<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.code == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.code) as u64) }
        + if self.message == "" { 0 } else { 1 + sizeof_len((&self.message).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.code != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.code))?; }
        if self.message != "" { w.write_with_tag(18, |w| w.write_string(&**&self.message))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Success { }

impl<'a> MessageRead<'a> for Success {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for Success { }

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Request<'a> {
    pub request: mod_Request::OneOfrequest<'a>,
}

impl<'a> MessageRead<'a> for Request<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.request = mod_Request::OneOfrequest::random_number(r.read_message::<random_number::RandomNumberRequest>(bytes)?),
                Ok(18) => msg.request = mod_Request::OneOfrequest::device_name(r.read_message::<bitbox02_system::SetDeviceNameRequest>(bytes)?),
                Ok(26) => msg.request = mod_Request::OneOfrequest::device_language(r.read_message::<bitbox02_system::SetDeviceLanguageRequest>(bytes)?),
                Ok(34) => msg.request = mod_Request::OneOfrequest::device_info(r.read_message::<bitbox02_system::DeviceInfoRequest>(bytes)?),
                Ok(42) => msg.request = mod_Request::OneOfrequest::set_password(r.read_message::<bitbox02_system::SetPasswordRequest>(bytes)?),
                Ok(50) => msg.request = mod_Request::OneOfrequest::create_backup(r.read_message::<backup_commands::CreateBackupRequest>(bytes)?),
                Ok(58) => msg.request = mod_Request::OneOfrequest::show_mnemonic(r.read_message::<mnemonic::ShowMnemonicRequest>(bytes)?),
                Ok(66) => msg.request = mod_Request::OneOfrequest::btc_pub(r.read_message::<btc::BTCPubRequest>(bytes)?),
                Ok(74) => msg.request = mod_Request::OneOfrequest::btc_sign_init(r.read_message::<btc::BTCSignInitRequest>(bytes)?),
                Ok(82) => msg.request = mod_Request::OneOfrequest::btc_sign_input(r.read_message::<btc::BTCSignInputRequest>(bytes)?),
                Ok(90) => msg.request = mod_Request::OneOfrequest::btc_sign_output(r.read_message::<btc::BTCSignOutputRequest>(bytes)?),
                Ok(98) => msg.request = mod_Request::OneOfrequest::insert_remove_sdcard(r.read_message::<bitbox02_system::InsertRemoveSDCardRequest>(bytes)?),
                Ok(106) => msg.request = mod_Request::OneOfrequest::check_sdcard(r.read_message::<bitbox02_system::CheckSDCardRequest>(bytes)?),
                Ok(114) => msg.request = mod_Request::OneOfrequest::set_mnemonic_passphrase_enabled(r.read_message::<mnemonic::SetMnemonicPassphraseEnabledRequest>(bytes)?),
                Ok(122) => msg.request = mod_Request::OneOfrequest::list_backups(r.read_message::<backup_commands::ListBackupsRequest>(bytes)?),
                Ok(130) => msg.request = mod_Request::OneOfrequest::restore_backup(r.read_message::<backup_commands::RestoreBackupRequest>(bytes)?),
                Ok(138) => msg.request = mod_Request::OneOfrequest::perform_attestation(r.read_message::<perform_attestation::PerformAttestationRequest>(bytes)?),
                Ok(146) => msg.request = mod_Request::OneOfrequest::reboot(r.read_message::<system::RebootRequest>(bytes)?),
                Ok(154) => msg.request = mod_Request::OneOfrequest::check_backup(r.read_message::<backup_commands::CheckBackupRequest>(bytes)?),
                Ok(162) => msg.request = mod_Request::OneOfrequest::eth(r.read_message::<eth::ETHRequest>(bytes)?),
                Ok(170) => msg.request = mod_Request::OneOfrequest::reset(r.read_message::<bitbox02_system::ResetRequest>(bytes)?),
                Ok(178) => msg.request = mod_Request::OneOfrequest::restore_from_mnemonic(r.read_message::<mnemonic::RestoreFromMnemonicRequest>(bytes)?),
                Ok(186) => msg.request = mod_Request::OneOfrequest::bitboxbase(r.read_message::<bitboxbase::BitBoxBaseRequest>(bytes)?),
                Ok(194) => msg.request = mod_Request::OneOfrequest::fingerprint(r.read_message::<common::RootFingerprintRequest>(bytes)?),
                Ok(202) => msg.request = mod_Request::OneOfrequest::btc(r.read_message::<btc::BTCRequest>(bytes)?),
                Ok(210) => msg.request = mod_Request::OneOfrequest::electrum_encryption_key(r.read_message::<keystore::ElectrumEncryptionKeyRequest>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Request<'a> {
    fn get_size(&self) -> usize {
        0
        + match self.request {
            mod_Request::OneOfrequest::random_number(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::device_name(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::device_language(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::device_info(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::set_password(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::create_backup(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::show_mnemonic(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::btc_pub(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::btc_sign_init(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::btc_sign_input(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::btc_sign_output(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::insert_remove_sdcard(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::check_sdcard(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::set_mnemonic_passphrase_enabled(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::list_backups(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::restore_backup(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::perform_attestation(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::reboot(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::check_backup(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::eth(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::reset(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::restore_from_mnemonic(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::bitboxbase(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::fingerprint(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::btc(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::electrum_encryption_key(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Request::OneOfrequest::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.request {            mod_Request::OneOfrequest::random_number(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::device_name(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::device_language(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::device_info(ref m) => { w.write_with_tag(34, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::set_password(ref m) => { w.write_with_tag(42, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::create_backup(ref m) => { w.write_with_tag(50, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::show_mnemonic(ref m) => { w.write_with_tag(58, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::btc_pub(ref m) => { w.write_with_tag(66, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::btc_sign_init(ref m) => { w.write_with_tag(74, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::btc_sign_input(ref m) => { w.write_with_tag(82, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::btc_sign_output(ref m) => { w.write_with_tag(90, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::insert_remove_sdcard(ref m) => { w.write_with_tag(98, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::check_sdcard(ref m) => { w.write_with_tag(106, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::set_mnemonic_passphrase_enabled(ref m) => { w.write_with_tag(114, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::list_backups(ref m) => { w.write_with_tag(122, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::restore_backup(ref m) => { w.write_with_tag(130, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::perform_attestation(ref m) => { w.write_with_tag(138, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::reboot(ref m) => { w.write_with_tag(146, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::check_backup(ref m) => { w.write_with_tag(154, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::eth(ref m) => { w.write_with_tag(162, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::reset(ref m) => { w.write_with_tag(170, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::restore_from_mnemonic(ref m) => { w.write_with_tag(178, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::bitboxbase(ref m) => { w.write_with_tag(186, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::fingerprint(ref m) => { w.write_with_tag(194, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::btc(ref m) => { w.write_with_tag(202, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::electrum_encryption_key(ref m) => { w.write_with_tag(210, |w| w.write_message(m))? },
            mod_Request::OneOfrequest::None => {},
    }        Ok(())
    }
}

pub mod mod_Request {

use alloc::vec::Vec;
use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfrequest<'a> {
    random_number(random_number::RandomNumberRequest),
    device_name(bitbox02_system::SetDeviceNameRequest<'a>),
    device_language(bitbox02_system::SetDeviceLanguageRequest<'a>),
    device_info(bitbox02_system::DeviceInfoRequest),
    set_password(bitbox02_system::SetPasswordRequest<'a>),
    create_backup(backup_commands::CreateBackupRequest),
    show_mnemonic(mnemonic::ShowMnemonicRequest),
    btc_pub(btc::BTCPubRequest<'a>),
    btc_sign_init(btc::BTCSignInitRequest<'a>),
    btc_sign_input(btc::BTCSignInputRequest<'a>),
    btc_sign_output(btc::BTCSignOutputRequest<'a>),
    insert_remove_sdcard(bitbox02_system::InsertRemoveSDCardRequest),
    check_sdcard(bitbox02_system::CheckSDCardRequest),
    set_mnemonic_passphrase_enabled(mnemonic::SetMnemonicPassphraseEnabledRequest),
    list_backups(backup_commands::ListBackupsRequest),
    restore_backup(backup_commands::RestoreBackupRequest<'a>),
    perform_attestation(perform_attestation::PerformAttestationRequest<'a>),
    reboot(system::RebootRequest),
    check_backup(backup_commands::CheckBackupRequest),
    eth(eth::ETHRequest<'a>),
    reset(bitbox02_system::ResetRequest),
    restore_from_mnemonic(mnemonic::RestoreFromMnemonicRequest),
    bitboxbase(bitboxbase::BitBoxBaseRequest<'a>),
    fingerprint(common::RootFingerprintRequest),
    btc(btc::BTCRequest<'a>),
    electrum_encryption_key(keystore::ElectrumEncryptionKeyRequest),
    None,
}

impl<'a> Default for OneOfrequest<'a> {
    fn default() -> Self {
        OneOfrequest::None
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response<'a> {
    pub response: mod_Response::OneOfresponse<'a>,
}

impl<'a> MessageRead<'a> for Response<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.response = mod_Response::OneOfresponse::success(r.read_message::<Success>(bytes)?),
                Ok(18) => msg.response = mod_Response::OneOfresponse::error(r.read_message::<Error>(bytes)?),
                Ok(26) => msg.response = mod_Response::OneOfresponse::random_number(r.read_message::<random_number::RandomNumberResponse>(bytes)?),
                Ok(34) => msg.response = mod_Response::OneOfresponse::device_info(r.read_message::<bitbox02_system::DeviceInfoResponse>(bytes)?),
                Ok(42) => msg.response = mod_Response::OneOfresponse::pub_pb(r.read_message::<common::PubResponse>(bytes)?),
                Ok(50) => msg.response = mod_Response::OneOfresponse::btc_sign_next(r.read_message::<btc::BTCSignNextResponse>(bytes)?),
                Ok(58) => msg.response = mod_Response::OneOfresponse::list_backups(r.read_message::<backup_commands::ListBackupsResponse>(bytes)?),
                Ok(66) => msg.response = mod_Response::OneOfresponse::check_backup(r.read_message::<backup_commands::CheckBackupResponse>(bytes)?),
                Ok(74) => msg.response = mod_Response::OneOfresponse::perform_attestation(r.read_message::<perform_attestation::PerformAttestationResponse>(bytes)?),
                Ok(82) => msg.response = mod_Response::OneOfresponse::check_sdcard(r.read_message::<bitbox02_system::CheckSDCardResponse>(bytes)?),
                Ok(90) => msg.response = mod_Response::OneOfresponse::eth(r.read_message::<eth::ETHResponse>(bytes)?),
                Ok(98) => msg.response = mod_Response::OneOfresponse::fingerprint(r.read_message::<common::RootFingerprintResponse>(bytes)?),
                Ok(106) => msg.response = mod_Response::OneOfresponse::btc(r.read_message::<btc::BTCResponse>(bytes)?),
                Ok(114) => msg.response = mod_Response::OneOfresponse::electrum_encryption_key(r.read_message::<keystore::ElectrumEncryptionKeyResponse>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Response<'a> {
    fn get_size(&self) -> usize {
        0
        + match self.response {
            mod_Response::OneOfresponse::success(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Response::OneOfresponse::error(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Response::OneOfresponse::random_number(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Response::OneOfresponse::device_info(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Response::OneOfresponse::pub_pb(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Response::OneOfresponse::btc_sign_next(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Response::OneOfresponse::list_backups(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Response::OneOfresponse::check_backup(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Response::OneOfresponse::perform_attestation(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Response::OneOfresponse::check_sdcard(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Response::OneOfresponse::eth(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Response::OneOfresponse::fingerprint(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Response::OneOfresponse::btc(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Response::OneOfresponse::electrum_encryption_key(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Response::OneOfresponse::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.response {            mod_Response::OneOfresponse::success(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            mod_Response::OneOfresponse::error(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_Response::OneOfresponse::random_number(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            mod_Response::OneOfresponse::device_info(ref m) => { w.write_with_tag(34, |w| w.write_message(m))? },
            mod_Response::OneOfresponse::pub_pb(ref m) => { w.write_with_tag(42, |w| w.write_message(m))? },
            mod_Response::OneOfresponse::btc_sign_next(ref m) => { w.write_with_tag(50, |w| w.write_message(m))? },
            mod_Response::OneOfresponse::list_backups(ref m) => { w.write_with_tag(58, |w| w.write_message(m))? },
            mod_Response::OneOfresponse::check_backup(ref m) => { w.write_with_tag(66, |w| w.write_message(m))? },
            mod_Response::OneOfresponse::perform_attestation(ref m) => { w.write_with_tag(74, |w| w.write_message(m))? },
            mod_Response::OneOfresponse::check_sdcard(ref m) => { w.write_with_tag(82, |w| w.write_message(m))? },
            mod_Response::OneOfresponse::eth(ref m) => { w.write_with_tag(90, |w| w.write_message(m))? },
            mod_Response::OneOfresponse::fingerprint(ref m) => { w.write_with_tag(98, |w| w.write_message(m))? },
            mod_Response::OneOfresponse::btc(ref m) => { w.write_with_tag(106, |w| w.write_message(m))? },
            mod_Response::OneOfresponse::electrum_encryption_key(ref m) => { w.write_with_tag(114, |w| w.write_message(m))? },
            mod_Response::OneOfresponse::None => {},
    }        Ok(())
    }
}

pub mod mod_Response {

use alloc::vec::Vec;
use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfresponse<'a> {
    success(Success),
    error(Error<'a>),
    random_number(random_number::RandomNumberResponse<'a>),
    device_info(bitbox02_system::DeviceInfoResponse<'a>),
    pub_pb(common::PubResponse<'a>),
    btc_sign_next(btc::BTCSignNextResponse<'a>),
    list_backups(backup_commands::ListBackupsResponse<'a>),
    check_backup(backup_commands::CheckBackupResponse<'a>),
    perform_attestation(perform_attestation::PerformAttestationResponse<'a>),
    check_sdcard(bitbox02_system::CheckSDCardResponse),
    eth(eth::ETHResponse<'a>),
    fingerprint(common::RootFingerprintResponse<'a>),
    btc(btc::BTCResponse<'a>),
    electrum_encryption_key(keystore::ElectrumEncryptionKeyResponse<'a>),
    None,
}

impl<'a> Default for OneOfresponse<'a> {
    fn default() -> Self {
        OneOfresponse::None
    }
}

}

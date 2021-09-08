// Copyright 2020 Shift Crypto AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::pb;

use crate::workflow::unlock::UnlockError;

extern crate alloc;
use alloc::string::String;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ErrorKind {
    InvalidInput,
    Memory,
    Generic,
    UserAbort,
    InvalidState,
    Disabled,
    Duplicate,
    NoiseEncrypt,
    NoiseDecrypt,
}

#[derive(Debug, PartialEq)]
pub struct Error {
    pub msg: Option<String>,
    pub kind: ErrorKind,
}

pub trait Context<A> {
    fn context(self, msg: String) -> Result<A, Error>;

    fn error_kind(self, kind: ErrorKind) -> Result<A, Error>;
}

pub trait IntoKind {
    fn kind(&self) -> ErrorKind;
}

impl IntoKind for () {
    fn kind(&self) -> ErrorKind {
        ErrorKind::Generic
    }
}

impl IntoKind for Error {
    fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl<A, E: core::fmt::Debug> Context<A> for Result<A, E> {
    fn context(self, msg: String) -> Result<A, Error> {
        self.map_err(|e| Error {
            msg: Some(format!("{}: {:?}", msg, e)),
            kind: ErrorKind::Generic,
        })
    }

    fn error_kind(self, kind: ErrorKind) -> Result<A, Error> {
        self.map_err(|e| Error {
            msg: Some(format!("{:?}", e)),
            kind,
        })
    }
}

impl<A> Context<A> for Option<A> {
    fn context(self, msg: String) -> Result<A, Error> {
        self.ok_or(Error {
            msg: Some(msg),
            kind: ErrorKind::Generic,
        })
    }

    fn error_kind(self, kind: ErrorKind) -> Result<A, Error> {
        self.ok_or(Error { msg: None, kind })
    }
}

impl core::convert::From<()> for Error {
    fn from(_error: ()) -> Self {
        Error {
            msg: None,
            kind: ErrorKind::Generic,
        }
    }
}

impl core::convert::From<bitbox02::memory::Error> for Error {
    fn from(_error: bitbox02::memory::Error) -> Self {
        Error {
            msg: None,
            kind: ErrorKind::Memory,
        }
    }
}

impl core::convert::From<crate::workflow::cancel::Error> for Error {
    fn from(_error: crate::workflow::cancel::Error) -> Self {
        Error {
            msg: None,
            kind: ErrorKind::UserAbort,
        }
    }
}

impl core::convert::From<crate::workflow::confirm::UserAbort> for Error {
    fn from(_error: crate::workflow::confirm::UserAbort) -> Self {
        Error {
            msg: None,
            kind: ErrorKind::UserAbort,
        }
    }
}

impl core::convert::From<crate::workflow::transaction::UserAbort> for Error {
    fn from(_error: crate::workflow::transaction::UserAbort) -> Self {
        Error {
            msg: None,
            kind: ErrorKind::UserAbort,
        }
    }
}

impl core::convert::From<crate::workflow::verify_message::Error> for Error {
    fn from(error: crate::workflow::verify_message::Error) -> Self {
        match error {
            crate::workflow::verify_message::Error::InvalidInput => Error {
                msg: None,
                kind: ErrorKind::InvalidInput,
            },
            crate::workflow::verify_message::Error::UserAbort => Error {
                msg: None,
                kind: ErrorKind::UserAbort,
            },
        }
    }
}

impl core::convert::From<UnlockError> for Error {
    fn from(error: UnlockError) -> Self {
        match error {
            UnlockError::UserAbort => Error {
                msg: None,
                kind: ErrorKind::UserAbort,
            },
            UnlockError::IncorrectPassword | UnlockError::Generic => Error {
                msg: None,
                kind: ErrorKind::Generic,
            },
        }
    }
}

use pb::response::Response;

/// Creates an Error response. Corresponds to commander.c:_report_error().
pub fn make_error(err: Error) -> Response {
    use ErrorKind::*;
    let err = match err.kind {
        InvalidInput => pb::Error {
            code: 101,
            message: format!("invalid input: {}", err.msg.unwrap_or("".into())),
        },
        Memory => pb::Error {
            code: 102,
            message: format!("memory: {}", err.msg.unwrap_or("".into())),
        },
        Generic => pb::Error {
            code: 103,
            message: format!("generic error: {}", err.msg.unwrap_or("".into())),
        },
        UserAbort => pb::Error {
            code: 104,
            message: format!("aborted by the user: {}", err.msg.unwrap_or("".into())),
        },
        InvalidState => pb::Error {
            code: 105,
            message: format!(
                "can't call this endpoint: wrong state: {}",
                err.msg.unwrap_or("".into())
            ),
        },
        Disabled => pb::Error {
            code: 106,
            message: format!("function disabled: {}", err.msg.unwrap_or("".into())),
        },
        Duplicate => pb::Error {
            code: 107,
            message: format!("duplicate entry: {}", err.msg.unwrap_or("".into())),
        },
        NoiseEncrypt => pb::Error {
            code: 108,
            message: format!("noise encryption failed: {}", err.msg.unwrap_or("".into())),
        },
        NoiseDecrypt => pb::Error {
            code: 109,
            message: format!("noise decryption failed: {}", err.msg.unwrap_or("".into())),
        },
    };
    Response::Error(err)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_context() {
        // let a: Result<bool, bool> = Err(false);
        // let b: Result<bool, Error> = a.context("LOL".into());
        // assert_eq!(&b.unwrap_err().msg, "LOL:");
    }
}

// Copyright 2020-2022 Jorge C. Leitão
// Copyright 2021 Datafuse Labs
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// Defines [`Error`], representing all errors returned by this crate.
use std::fmt::Debug;
/// Defines [`Error`], representing all errors returned by this crate.
use std::fmt::Display;
/// Defines [`Error`], representing all errors returned by this crate.
use std::fmt::Formatter;

use databend_common_exception::ErrorCode;

/// Enum with all errors in this crate.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Returned when functionality is not yet available.
    NotYetImplemented(String),
    /// Wrapper for an error triggered by a dependency
    External(String, Box<dyn std::error::Error + Send + Sync>),
    /// Wrapper for IO errors
    Io(std::io::Error),
    /// When an invalid argument is passed to a function.
    InvalidArgumentError(String),
    /// Error during import or export to/from a format
    ExternalFormat(String),
    /// Whenever pushing to a container fails because it does not support more entries.
    /// The solution is usually to use a higher-capacity container-backing type.
    Overflow,
    /// Whenever incoming data from the C data interface, IPC or Flight does not fulfil the Arrow specification.
    OutOfSpec(String),
}

impl Error {
    /// Wraps an external error in an `Error`.
    pub fn from_external_error(error: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::External("".to_string(), Box::new(error))
    }

    pub(crate) fn oos<A: Into<String>>(msg: A) -> Self {
        Self::OutOfSpec(msg.into())
    }

    #[allow(dead_code)]
    pub(crate) fn nyi<A: Into<String>>(msg: A) -> Self {
        Self::NotYetImplemented(msg.into())
    }
}

impl From<::std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(error: std::str::Utf8Error) -> Self {
        Error::External("".to_string(), Box::new(error))
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(error: std::string::FromUtf8Error) -> Self {
        Error::External("".to_string(), Box::new(error))
    }
}

impl From<simdutf8::basic::Utf8Error> for Error {
    fn from(error: simdutf8::basic::Utf8Error) -> Self {
        Error::External("".to_string(), Box::new(error))
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::External("".to_string(), Box::new(error))
    }
}

impl From<std::collections::TryReserveError> for Error {
    fn from(_: std::collections::TryReserveError) -> Error {
        Error::Overflow
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Error::NotYetImplemented(source) => {
                write!(f, "Not yet implemented: {}", &source)
            }
            Error::External(message, source) => {
                write!(f, "External error{}: {}", message, &source)
            }
            Error::Io(desc) => write!(f, "Io error: {desc}"),
            Error::InvalidArgumentError(desc) => {
                write!(f, "Invalid argument error: {desc}")
            }
            Error::ExternalFormat(desc) => {
                write!(f, "External format error: {desc}")
            }
            Error::Overflow => {
                write!(f, "Operation overflew the backing container.")
            }
            Error::OutOfSpec(message) => {
                write!(f, "{message}")
            }
        }
    }
}

impl std::error::Error for Error {}

/// Typedef for a [`std::result::Result`] of an [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

impl From<Error> for ErrorCode {
    fn from(error: Error) -> Self {
        match error {
            Error::NotYetImplemented(v) => ErrorCode::Unimplemented(format!("arrow: {v}")),
            v => ErrorCode::from_std_error(v),
        }
    }
}

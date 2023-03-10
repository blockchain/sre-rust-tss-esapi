// Copyright 2022 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0
use crate::{constants::BaseError, error::BaseReturnCode, Error, Result, WrapperErrorKind};
use log::error;
use std::convert::TryFrom;

/// Enum representing the error return codes generated by the MUAPI layer
/// in TSS.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MuapiReturnCode {
    base_error: BaseError,
}

impl MuapiReturnCode {
    /// Returns the BaseError associated with the MUAPI return code.
    pub const fn base_error(&self) -> BaseError {
        self.base_error
    }
}

impl From<MuapiReturnCode> for BaseReturnCode {
    fn from(muapi_return_code: MuapiReturnCode) -> Self {
        muapi_return_code.base_error.into()
    }
}

impl TryFrom<BaseReturnCode> for MuapiReturnCode {
    type Error = Error;

    fn try_from(base_return_code: BaseReturnCode) -> Result<Self> {
        MuapiReturnCode::try_from(BaseError::from(base_return_code))
    }
}

impl TryFrom<u16> for MuapiReturnCode {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self> {
        MuapiReturnCode::try_from(BaseError::try_from(value)?)
    }
}

impl From<MuapiReturnCode> for u16 {
    fn from(muapi_error_code: MuapiReturnCode) -> Self {
        BaseReturnCode::from(muapi_error_code).into()
    }
}

impl TryFrom<BaseError> for MuapiReturnCode {
    type Error = Error;

    fn try_from(base_error: BaseError) -> Result<Self> {
        match base_error {
            BaseError::GeneralFailure
            | BaseError::BadReference
            | BaseError::BadSize
            | BaseError::BadValue
            | BaseError::InsufficientBuffer => Ok(MuapiReturnCode { base_error }),
            _ => {
                error!(
                    "{} is not a valid MuapiReturnCode base error",
                    u16::from(base_error)
                );
                Err(Error::local_error(WrapperErrorKind::InvalidParam))
            }
        }
    }
}

impl From<MuapiReturnCode> for BaseError {
    fn from(muapi_return_code: MuapiReturnCode) -> Self {
        muapi_return_code.base_error
    }
}

impl std::error::Error for MuapiReturnCode {}

impl std::fmt::Display for MuapiReturnCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", BaseReturnCode::from(*self))
    }
}

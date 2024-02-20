use std::fmt;
use std::fmt::Formatter;

use serde::Serialize;

use nihility_procmacro::Sign;

use crate::response_code::{Resp, RespCode};
use crate::utils::auth::Signature;

#[derive(Debug, Default, Serialize)]
pub enum ResponseCode {
    #[default]
    Success,
    UnknownError,
    UnableToProcess,
    AuthenticationFail,
}

#[derive(Default, Serialize, Sign)]
pub struct ResponseEntity {
    code: ResponseCode,
    sign: Vec<u8>,
}

impl ResponseEntity {
    pub fn success(&mut self) {
        self.code = ResponseCode::Success;
    }
    pub fn unknown_error(&mut self) {
        self.code = ResponseCode::UnknownError;
    }
    pub fn unable_to_process(&mut self) {
        self.code = ResponseCode::UnableToProcess;
    }
    pub fn authentication_fail(&mut self) {
        self.code = ResponseCode::AuthenticationFail;
    }
    pub fn code(&self) -> &ResponseCode {
        &self.code
    }
}

impl fmt::Debug for ResponseEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Response ( code: {:?} )", self.code)
    }
}

impl From<RespCode> for ResponseCode {
    fn from(value: RespCode) -> Self {
        match value {
            RespCode::UnknownError => ResponseCode::UnknownError,
            RespCode::Success => ResponseCode::Success,
            RespCode::UnableToProcess => ResponseCode::UnableToProcess,
            RespCode::AuthenticationFail => ResponseCode::AuthenticationFail,
        }
    }
}

impl From<ResponseCode> for RespCode {
    fn from(value: ResponseCode) -> Self {
        match value {
            ResponseCode::Success => RespCode::Success,
            ResponseCode::UnknownError => RespCode::UnknownError,
            ResponseCode::UnableToProcess => RespCode::UnableToProcess,
            ResponseCode::AuthenticationFail => RespCode::AuthenticationFail,
        }
    }
}

impl From<Resp> for ResponseEntity {
    fn from(value: Resp) -> Self {
        ResponseEntity {
            code: ResponseCode::from(value.code()),
            sign: value.sign,
        }
    }
}

impl From<ResponseEntity> for Resp {
    fn from(value: ResponseEntity) -> Self {
        Resp {
            code: RespCode::from(value.code).into(),
            sign: value.sign,
        }
    }
}

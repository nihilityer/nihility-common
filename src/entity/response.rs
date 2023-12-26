use crate::response_code::RespCode;

#[derive(Debug)]
pub enum ResponseCode {
    Success,
    UnknownError,
    UnableToProcess,
}

impl From<RespCode> for ResponseCode {
    fn from(value: RespCode) -> Self {
        match value {
            RespCode::UnknownError => ResponseCode::UnknownError,
            RespCode::Success => ResponseCode::Success,
            RespCode::UnableToProcess => ResponseCode::UnableToProcess,
        }
    }
}

impl From<ResponseCode> for RespCode {
    fn from(value: ResponseCode) -> Self {
        match value {
            ResponseCode::Success => RespCode::Success,
            ResponseCode::UnknownError => RespCode::UnknownError,
            ResponseCode::UnableToProcess => RespCode::UnableToProcess,
        }
    }
}

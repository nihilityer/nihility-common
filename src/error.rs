use std::net::AddrParseError;

use thiserror::Error;

use crate::entity::instruct::InstructData;
use crate::entity::manipulate::ManipulateData;

pub type WrapResult<T> = Result<T, NihilityCommonError>;

#[derive(Error, Debug)]
pub enum NihilityCommonError {
    #[error("The `{module:?}` Module Config {param:?} Error")]
    Config {
        module: String,
        param: String,
    },
    #[error("This Manipulate Entity Is In Other Type, Please Create {0:?} Type Req")]
    CreateManipulateReq(ManipulateData),
    #[error("This Instruct Entity Is In Other Type, Please Create {0:?} Type Req")]
    CreateInstructReq(InstructData),
    #[error("Mock Client Cannot {0:?}")]
    RefusalToProcess(String),
    #[error("Parse Addr Error")]
    AddrParse(#[from] AddrParseError),
    #[error("Tonic Transport Error")]
    Tonic(#[from] tonic::transport::Error),
    #[error("Tonic Status")]
    Status(#[from] tonic::Status),
    #[error("Unknown Error")]
    Unknown,
}


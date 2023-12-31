use std::net::AddrParseError;

use thiserror::Error;

use crate::entity::manipulate::ManipulateData;
use crate::entity::submodule::OperateType;

pub type WrapResult<T> = Result<T, NihilityCommonError>;

#[derive(Error, Debug)]
pub enum NihilityCommonError {
    #[error("This Manipulate Entity Is In Other Type, Please Create {0:?} Type Req")]
    CreateManipulateReq(ManipulateData),
    #[error("This Module Operate Don't Have Info")]
    CreateSubmoduleReq,
    #[error("This SubmoduleReq Don't Have ConnectionParams")]
    CreateModuleOperate,
    #[error("This DirectConnectionManipulate Don't Have ConnectionParams")]
    CreateManipulateEntity,
    #[error("This Module Operate Is In Other Type, Please Create {0:?} Type Req")]
    CreateSubmoduleHeartbeat(OperateType),
    #[error("{0:?} Client Not Connected")]
    NotConnected(String),
    #[error("Config Field Missing")]
    ConfigFieldMissing,
    #[error("Parse Addr Error")]
    AddrParse(#[from] AddrParseError),
    #[error("Tonic Transport Error")]
    Tonic(#[from] tonic::transport::Error),
    #[error("Tonic Status")]
    Status(#[from] tonic::Status),
}

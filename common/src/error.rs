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
    #[error("Auth Id Not Exist")]
    AuthId,
    #[error("This File Not Exist: {0:?}")]
    FileNotExist(String),
    #[error("{0:?} Client Not Connected: {0}")]
    NotConnected(String),
    #[error("Config Field Missing")]
    ConfigFieldMissing,
    #[error("Std IO Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("FromUtf8Error: {0}")]
    FromUtf8(#[from] std::string::FromUtf8Error),
    #[error("Postcard: {0}")]
    Postcard(#[from] postcard::Error),
    #[error("Parse Addr Error: {0}")]
    AddrParse(#[from] std::net::AddrParseError),
    #[error("Tonic Transport Error: {0}")]
    Tonic(#[from] tonic::transport::Error),
    #[error("Tonic Status: {0}")]
    Status(#[from] tonic::Status),
    #[error("Rsa Error: {0}")]
    Rsa(#[from] rsa::Error),
    #[error("Rsa Pkcs8 Error: {0}")]
    RsaPkcs8(#[from] rsa::pkcs8::Error),
    #[error("Rsa Spki Error: {0}")]
    RsaSpki(#[from] rsa::pkcs8::spki::Error),
}

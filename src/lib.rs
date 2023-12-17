use std::sync::OnceLock;

use tokio_util::sync::CancellationToken;

pub use communicat::grpc::client::GrpcClient;
pub use communicat::grpc::config::{GrpcClientConfig, GrpcServerConfig};
pub use communicat::grpc::server::GrpcServer;
pub use communicat::NihilityClient;
pub use communicat::NihilityServer;
pub use entity::instruct::{
    InstructData,
    InstructEntity,
    InstructInfoEntity,
    InstructType,
};
pub use entity::manipulate::{
    ManipulateData,
    ManipulateEntity,
    ManipulateInfoEntity,
    ManipulateType,
};
pub use entity::response::ResponseCode;
pub use entity::submodule::{
    ClientType,
    ConnectionType,
    ModuleOperate,
    OperateType,
    SubmoduleInfo,
};

mod communicat;
mod entity;
mod error;

pub(crate) static CANCELLATION_TOKEN: OnceLock<CancellationToken> = OnceLock::new();

pub(crate) static DEFAULT_RECEIVER_SUBMODULE_NAME: OnceLock<String> = OnceLock::new();

pub(crate) static DEFAULT_INSTRUCT_HANDLER_SUBMODULE_NAME: OnceLock<String> = OnceLock::new();

pub(crate) mod manipulate {
    tonic::include_proto!("manipulate");
}

pub(crate) mod instruct {
    tonic::include_proto!("instruct");
}

pub(crate) mod submodule {
    tonic::include_proto!("submodule");
}

pub(crate) mod response_code {
    tonic::include_proto!("response_code");
}

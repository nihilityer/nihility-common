pub use communicat::grpc::{
    client::GrpcClient,
    config::{GrpcClientConfig, GrpcServerConfig},
    server::GrpcServer,
};
pub use communicat::NihilityClient;
pub use communicat::NihilityServer;
pub use entity::instruct::{InstructData, InstructEntity, InstructInfoEntity, InstructType};
pub use entity::manipulate::{
    ManipulateData, ManipulateEntity, ManipulateInfoEntity, ManipulateType,
};
pub use entity::response::ResponseCode;
pub use entity::submodule::{
    ClientType, ConnParams, ConnectionType, ModuleOperate, OperateType, SubmoduleInfo,
};
pub use utils::auth::{core_authentication_core_init, submodule_authentication_core_init, set_entity_submodule_sign};

mod communicat;
mod entity;
mod error;
mod utils;

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

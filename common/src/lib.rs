use std::sync::OnceLock;

use tracing::debug;

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
pub use entity::module_operate::{
    ClientType, ConnParams, ConnectionType, ModuleOperate, OperateType, SubmoduleInfo,
};
pub use entity::response::ResponseCode;
pub use utils::{
    auth::{core_authentication_core_init, remove_submodule_public_key, set_core_public_key_path},
    log::{Log, LogConfig, LogOutType},
};

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

static SUBMODULE_NAME: OnceLock<String> = OnceLock::new();

pub fn set_submodule_name(name: String) {
    SUBMODULE_NAME.get_or_init(|| name);
}

pub fn get_submodule_name() -> String {
    match SUBMODULE_NAME.get() {
        None => {
            debug!("Submodule Name Not Init, If Core Use This, Ignore This Error Info");
            String::new()
        }
        Some(name) => name.to_string(),
    }
}

use std::sync::OnceLock;

use tracing::error;

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
    auth::{
        core_authentication_core_init, get_auth_id, remove_submodule_public_key, set_auth_id,
        set_core_public_key_path,
    },
    log::{Log, LogConfig, LogLevel, LogOutType},
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

static DEFAULT_RECEIVER_SUBMODULE: OnceLock<String> = OnceLock::new();

static CORE_FLAG: OnceLock<bool> = OnceLock::new();

pub fn set_submodule_name(name: &str) {
    SUBMODULE_NAME.get_or_init(|| name.to_string());
}

fn get_submodule_name() -> String {
    match SUBMODULE_NAME.get() {
        None => {
            match CORE_FLAG.get() {
                Some(true) => {}
                _ => {
                    error!("Submodule Name Not Init!");
                }
            }
            String::new()
        }
        Some(name) => name.to_string(),
    }
}

pub fn set_default_receiver_submodule(submodule_name: &str) {
    DEFAULT_RECEIVER_SUBMODULE.get_or_init(|| submodule_name.to_string());
}

fn get_default_receiver_submodule() -> String {
    match DEFAULT_RECEIVER_SUBMODULE.get() {
        None => {
            match CORE_FLAG.get() {
                Some(true) => {}
                _ => {
                    error!("Default Receiver Submodule Name Not Init!");
                }
            }
            String::new()
        }
        Some(submodule_name) => submodule_name.to_string(),
    }
}

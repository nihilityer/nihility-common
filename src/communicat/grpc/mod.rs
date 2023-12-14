use tokio::spawn;
use tokio::sync::mpsc::UnboundedSender;
use tracing::error;

use crate::CANCELLATION_TOKEN;
use crate::communicat::grpc::config::GrpcConfig;
use crate::entity::instruct::InstructEntity;
use crate::entity::manipulate::ManipulateEntity;
use crate::entity::submodule::ModuleOperate;

mod client;
mod server;
mod config;

pub(super) fn start(
    grpc_config: GrpcConfig,
    communicat_status_sender: UnboundedSender<String>,
    operate_module_sender: UnboundedSender<ModuleOperate>,
    instruct_sender: UnboundedSender<InstructEntity>,
    manipulate_sender: UnboundedSender<ManipulateEntity>,
) {
    spawn(async move {
        if let Err(e) = server::start_server(
            grpc_config,
            operate_module_sender,
            instruct_sender,
            manipulate_sender,
        )
            .await
        {
            error!("Grpc Server Error: {}", e);
            CANCELLATION_TOKEN.get().unwrap().cancel();
        }
        communicat_status_sender
            .send("Grpc Server".to_string())
            .unwrap();
    });
}

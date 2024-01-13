use std::net::IpAddr;
use std::pin::Pin;

use async_trait::async_trait;
use tokio::spawn;
use tokio::sync::mpsc::UnboundedSender;
use tokio_util::sync::CancellationToken;
use tonic::codegen::tokio_stream::Stream;
use tonic::transport::Server;
use tonic::Status;
use tracing::{error, info};

use crate::communicat::grpc::config::GrpcServerConfig;
use crate::communicat::grpc::server::instruct::InstructImpl;
use crate::communicat::grpc::server::manipulate::ManipulateImpl;
use crate::communicat::grpc::server::submodule::SubmoduleImpl;
use crate::communicat::NihilityServer;
use crate::entity::instruct::InstructEntity;
use crate::entity::manipulate::ManipulateEntity;
use crate::entity::submodule::ModuleOperate;
use crate::error::WrapResult;
use crate::instruct::instruct_server::InstructServer;
use crate::manipulate::manipulate_server::ManipulateServer;
use crate::response_code::Resp;
use crate::submodule::submodule_server::SubmoduleServer;

mod instruct;
mod manipulate;
mod submodule;

type StreamResp = Pin<Box<dyn Stream<Item = Result<Resp, Status>> + Send>>;

pub struct GrpcServer {
    server_config: GrpcServerConfig,
    cancellation_token: CancellationToken,
    submodule_operate_server: Option<SubmoduleServer<SubmoduleImpl>>,
    instruct_server: Option<InstructServer<InstructImpl>>,
    manipulate_server: Option<ManipulateServer<ManipulateImpl>>,
}

impl GrpcServer {
    pub fn init(
        grpc_server_config: GrpcServerConfig,
        cancellation_token: CancellationToken,
    ) -> Self {
        GrpcServer {
            server_config: grpc_server_config,
            cancellation_token,
            submodule_operate_server: None,
            instruct_server: None,
            manipulate_server: None,
        }
    }
}

#[async_trait]
impl NihilityServer for GrpcServer {
    fn set_submodule_operate_sender(
        &mut self,
        submodule_sender: UnboundedSender<ModuleOperate>,
    ) -> WrapResult<()> {
        self.submodule_operate_server =
            Some(SubmoduleServer::new(SubmoduleImpl::init(submodule_sender)));
        Ok(())
    }

    fn set_instruct_sender(
        &mut self,
        instruct_sender: UnboundedSender<InstructEntity>,
    ) -> WrapResult<()> {
        self.instruct_server = Some(InstructServer::new(InstructImpl::init(instruct_sender)));
        Ok(())
    }

    fn set_manipulate_sender(
        &mut self,
        manipulate_sender: UnboundedSender<ManipulateEntity>,
    ) -> WrapResult<()> {
        self.manipulate_server = Some(ManipulateServer::new(ManipulateImpl::init(
            manipulate_sender,
        )));
        Ok(())
    }

    fn start(&mut self) -> WrapResult<()> {
        let bind_addr = match self.server_config.bind_ip {
            IpAddr::V4(ip) => format!("{}:{}", ip, self.server_config.bind_port),
            IpAddr::V6(ip) => format!("[{}]:{}", ip, self.server_config.bind_port),
        };
        info!("Grpc Server Bind At {}", &bind_addr);
        let server_cancellation_token = self.cancellation_token.clone();
        let server = Server::builder()
            .add_optional_service(self.submodule_operate_server.clone())
            .add_optional_service(self.instruct_server.clone())
            .add_optional_service(self.manipulate_server.clone())
            .serve_with_shutdown(bind_addr.parse()?, async move {
                server_cancellation_token.cancelled().await
            });
        let cancellation_token = self.cancellation_token.clone();
        spawn(async move {
            if let Err(e) = server.await {
                error!("Grpc Server Error: {}", e);
                cancellation_token.cancel();
            }
            info!("Grpc Server Stop")
        });
        Ok(())
    }
}

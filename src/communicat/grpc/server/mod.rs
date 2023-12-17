use std::net::IpAddr;
use std::pin::Pin;

use async_trait::async_trait;
use tokio::spawn;
use tokio::sync::mpsc::UnboundedSender;
use tonic::codegen::tokio_stream::Stream;
use tonic::Status;
use tonic::transport::Server;
use tracing::{error, info};

use crate::CANCELLATION_TOKEN;
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

type StreamResp = Pin<Box<dyn Stream<Item=Result<Resp, Status>> + Send>>;

pub struct GrpcServer {
    server_config: GrpcServerConfig,
    submodule_operate_server: Option<SubmoduleServer<SubmoduleImpl>>,
    instruct_server: Option<InstructServer<InstructImpl>>,
    manipulate_server: Option<ManipulateServer<ManipulateImpl>>,
}

#[async_trait]
impl NihilityServer<GrpcServerConfig> for GrpcServer {
    fn init(config: GrpcServerConfig) -> WrapResult<Self> where Self: Sized + Send + Sync {
        Ok(GrpcServer {
            server_config: config,
            submodule_operate_server: None,
            instruct_server: None,
            manipulate_server: None,
        })
    }

    fn set_submodule_operate_sender(&mut self, submodule_sender: UnboundedSender<ModuleOperate>) -> WrapResult<()> {
        self.submodule_operate_server = Some(SubmoduleServer::new(SubmoduleImpl::init(submodule_sender)));
        Ok(())
    }

    fn set_instruct_sender(&mut self, instruct_sender: UnboundedSender<InstructEntity>) -> WrapResult<()> {
        self.instruct_server = Some(InstructServer::new(InstructImpl::init(instruct_sender)));
        Ok(())
    }

    fn set_manipulate_sender(&mut self, manipulate_sender: UnboundedSender<ManipulateEntity>) -> WrapResult<()> {
        self.manipulate_server = Some(ManipulateServer::new(ManipulateImpl::init(manipulate_sender)));
        Ok(())
    }

    fn start(&mut self) -> WrapResult<()> {
        let bind_addr = match self.server_config.bind_ip {
            IpAddr::V4(ip) => format!("{}:{}", ip, self.server_config.bind_port),
            IpAddr::V6(ip) => format!("[{}]:{}", ip, self.server_config.bind_port)
        };
        info!("Grpc Server Bind At {}", &bind_addr);
        let server = Server::builder()
            .add_optional_service(self.submodule_operate_server.clone())
            .add_optional_service(self.instruct_server.clone())
            .add_optional_service(self.manipulate_server.clone())
            .serve_with_shutdown(bind_addr.parse()?, async move {
                CANCELLATION_TOKEN.get().unwrap().cancelled().await
            });
        spawn(async move {
            if let Err(e) = server.await {
                error!("Grpc Server Error: {}", e);
                CANCELLATION_TOKEN.get().unwrap().cancel();
            }
            info!("Grpc Server Stop")
        });
        Ok(())
    }
}

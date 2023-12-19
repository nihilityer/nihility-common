use async_trait::async_trait;
use tonic::transport::Channel;

use crate::communicat::{NihilityClient};
use crate::communicat::grpc::config::GrpcClientConfig;
use crate::error::WrapResult;
use crate::instruct::instruct_client::InstructClient;
use crate::manipulate::manipulate_client::ManipulateClient;
use crate::submodule::submodule_client::SubmoduleClient;

mod instruct;
mod manipulate;
mod submodule_operate;

pub struct GrpcClient {
    config: GrpcClientConfig,
    submodule_operate_client: Option<SubmoduleClient<Channel>>,
    instruct_client: Option<InstructClient<Channel>>,
    manipulate_client: Option<ManipulateClient<Channel>>,
}

impl GrpcClient {
    pub fn init(grpc_client_config: GrpcClientConfig) -> Self {
        GrpcClient {
            config: grpc_client_config,
            submodule_operate_client: None,
            instruct_client: None,
            manipulate_client: None,
        }
    }
}

#[async_trait]
impl NihilityClient for GrpcClient {
    async fn connection_submodule_operate_server(&mut self) -> WrapResult<()> {
        self.submodule_operate_client = Some(SubmoduleClient::connect(self.config.terminal_address.to_string()).await?);
        Ok(())
    }

    async fn connection_instruct_server(&mut self) -> WrapResult<()> {
        self.instruct_client = Some(InstructClient::connect(self.config.terminal_address.to_string()).await?);
        Ok(())
    }

    async fn connection_manipulate_server(&mut self) -> WrapResult<()> {
        self.manipulate_client = Some(ManipulateClient::connect(self.config.terminal_address.to_string()).await?);
        Ok(())
    }
}
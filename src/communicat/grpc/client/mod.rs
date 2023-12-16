use async_trait::async_trait;
use tonic::transport::Channel;

use crate::communicat::grpc::config::GrpcClientConfig;
use crate::communicat::NihilityClient;
use crate::error::WrapResult;
use crate::instruct::instruct_client::InstructClient;
use crate::manipulate::manipulate_client::ManipulateClient;

mod instruct;
mod manipulate;

pub struct GrpcClient {
    config: GrpcClientConfig,
    instruct_client: InstructClient<Channel>,
    manipulate_client: ManipulateClient<Channel>,
}

#[async_trait]
impl NihilityClient<GrpcClientConfig> for GrpcClient {
    async fn init(config: GrpcClientConfig) -> WrapResult<Self> where Self: Sized + Send + Sync {
        let instruct_client = InstructClient::connect(config.terminal_address.to_string()).await?;
        let manipulate_client = ManipulateClient::connect(config.terminal_address.to_string()).await?;
        Ok(GrpcClient {
            config,
            instruct_client,
            manipulate_client,
        })
    }
}
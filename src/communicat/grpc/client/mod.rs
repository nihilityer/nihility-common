use async_trait::async_trait;
use tonic::transport::Channel;

use crate::communicat::client::NihilityClient;
use crate::communicat::grpc::GrpcConfig;
use crate::instruct::instruct_client::InstructClient;
use crate::manipulate::manipulate_client::ManipulateClient;

mod instruct;
mod manipulate;

pub struct GrpcClient {
    config: GrpcConfig,
    instruct_client: InstructClient<Channel>,
    manipulate_client: ManipulateClient<Channel>,
}

#[async_trait]
impl NihilityClient<GrpcConfig> for GrpcClient {
    async fn init(config: GrpcConfig) -> Self {
        todo!()
    }
}
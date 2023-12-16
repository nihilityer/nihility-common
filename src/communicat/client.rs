use async_trait::async_trait;

use crate::communicat::{InitClientConfig, SendInstructOperate, SendManipulateOperate};

#[async_trait]
pub trait NihilityClient<T: InitClientConfig>: SendManipulateOperate + SendInstructOperate {
    async fn init(config: T) -> Self;
}
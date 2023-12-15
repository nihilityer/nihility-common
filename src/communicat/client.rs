use async_trait::async_trait;
use tokio::sync::mpsc::Receiver;

use crate::entity::instruct::InstructEntity;
use crate::entity::response::ResponseCode;
use crate::error::WrapResult;

#[async_trait]
pub(self) trait NihilityClient {
    async fn text_instruct(&mut self, instruct: InstructEntity) -> WrapResult<ResponseCode>;
    async fn multiple_text_instruct(
        &mut self,
        instruct_stream: Receiver<InstructEntity>,
    ) -> WrapResult<Receiver<ResponseCode>>;
}
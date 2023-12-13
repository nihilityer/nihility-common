use async_trait::async_trait;
use tokio::sync::mpsc::Receiver;
use tracing::warn;

use crate::communicat::{SendInstructOperate, SendManipulateOperate};
use crate::error::NihilityCommonError::RefusalToProcess;
use crate::error::WrapResult;
use crate::instruct::TextInstruct;
use crate::manipulate::{SimpleManipulate, TextDisplayManipulate};
use crate::response_code::RespCode;

#[derive(Default)]
pub struct MockInstructClient;

#[derive(Default)]
pub struct MockManipulateClient;

#[async_trait]
impl SendInstructOperate for MockInstructClient {
    async fn send_text(&mut self, instruct: TextInstruct) -> WrapResult<RespCode> {
        warn!("Mock Instruct Client Get Instruct: {:?}", instruct);
        return Ok(RespCode::UnableToProcess);
    }

    async fn send_multiple_text(
        &mut self,
        instruct_stream: Receiver<TextInstruct>,
    ) -> WrapResult<Receiver<RespCode>> {
        warn!("Mock Instruct Client Get Instruct: {:?}", instruct_stream);
        return Err(RefusalToProcess("send_multiple_text".to_string()));
    }
}

#[async_trait]
impl SendManipulateOperate for MockManipulateClient {
    async fn send_simple(&mut self, manipulate: SimpleManipulate) -> WrapResult<RespCode> {
        warn!("Mock Manipulate Client Get Manipulate: {:?}", manipulate);
        return Ok(RespCode::UnableToProcess);
    }

    async fn send_text_display(&mut self, manipulate: TextDisplayManipulate) -> WrapResult<RespCode> {
        warn!("Mock Manipulate Client Get Manipulate: {:?}", manipulate);
        return Ok(RespCode::UnableToProcess);
    }

    async fn send_multiple_text_display(
        &mut self,
        manipulate_stream: Receiver<TextDisplayManipulate>,
    ) -> WrapResult<Receiver<RespCode>> {
        warn!(
            "Mock Manipulate Client Get Manipulate: {:?}",
            manipulate_stream
        );
        return Err(RefusalToProcess("send_multiple_text_display".to_string()));
    }
}

mod grpc;
mod mock;

use tokio::sync::mpsc::Receiver;
use tonic::async_trait;
use crate::error::WrapResult;
use crate::instruct::TextInstruct;
use crate::manipulate::{SimpleManipulate, TextDisplayManipulate};
use crate::response_code::RespCode;

/// 发送指令特征
#[async_trait]
pub trait SendInstructOperate {
    async fn send_text(&mut self, instruct: TextInstruct) -> WrapResult<RespCode>;
    async fn send_multiple_text(
        &mut self,
        instruct_stream: Receiver<TextInstruct>,
    ) -> WrapResult<Receiver<RespCode>>;
}

/// 发送操作特征
#[async_trait]
pub trait SendManipulateOperate {
    async fn send_simple(&mut self, manipulate: SimpleManipulate) -> WrapResult<RespCode>;
    async fn send_text_display(&mut self, manipulate: TextDisplayManipulate) -> WrapResult<RespCode>;
    async fn send_multiple_text_display(
        &mut self,
        manipulate_stream: Receiver<TextDisplayManipulate>,
    ) -> WrapResult<Receiver<RespCode>>;
}
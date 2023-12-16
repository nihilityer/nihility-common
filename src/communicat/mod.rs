use std::path::Path;

use tokio::sync::mpsc::Receiver;
use tonic::async_trait;

use crate::entity::instruct::InstructEntity;
use crate::entity::response::ResponseCode;
use crate::error::WrapResult;
use crate::manipulate::{SimpleManipulate, TextDisplayManipulate};
use crate::response_code::RespCode;

mod grpc;
mod mock;
mod client;

pub trait InitClientConfig {
    fn read_from_toml_file<P: AsRef<Path>>(file_path: P) -> Self;
}

/// 发送指令特征
#[async_trait]
pub trait SendInstructOperate {
    async fn send_text(&mut self, instruct: InstructEntity) -> WrapResult<ResponseCode>;
    async fn send_multiple_text(
        &mut self,
        instruct_stream: Receiver<InstructEntity>,
    ) -> WrapResult<Receiver<ResponseCode>>;
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
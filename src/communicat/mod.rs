use tokio::sync::mpsc::{Receiver, UnboundedSender};
use tonic::async_trait;

use crate::entity::instruct::InstructEntity;
use crate::entity::manipulate::ManipulateEntity;
use crate::entity::response::ResponseCode;
use crate::entity::submodule::ModuleOperate;
use crate::error::WrapResult;
use crate::manipulate::{SimpleManipulate, TextDisplayManipulate};
use crate::response_code::RespCode;

pub mod grpc;
mod mock;

pub trait InitClientConfig: Default {}

#[async_trait]
pub trait NihilityClient<T: InitClientConfig>: SendManipulateOperate + SendInstructOperate {
    async fn init(config: T) -> WrapResult<Self> where Self: Sized + Send + Sync;
}

pub trait InitServerConfig: Default {}

#[async_trait]
pub trait NihilityServer<T: InitServerConfig> {
    fn init(config: T) -> WrapResult<Self> where Self: Sized + Send + Sync;

    fn set_submodule_operate_sender(&mut self, submodule_sender: UnboundedSender<ModuleOperate>) -> WrapResult<()>;

    fn set_instruct_sender(&mut self, instruct_sender: UnboundedSender<InstructEntity>) -> WrapResult<()>;

    fn set_manipulate_sender(&mut self, manipulate_sender: UnboundedSender<ManipulateEntity>) -> WrapResult<()>;

    fn start(&mut self) -> WrapResult<()>;
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
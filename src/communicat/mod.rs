use tokio::sync::mpsc::{Receiver, UnboundedSender};
use tonic::async_trait;

use crate::entity::instruct::InstructEntity;
use crate::entity::manipulate::ManipulateEntity;
use crate::entity::response::ResponseCode;
use crate::entity::submodule::ModuleOperate;
use crate::error::{NihilityCommonError, WrapResult};

pub mod grpc;

pub trait InitClientConfig: Default {}

#[async_trait]
pub trait NihilityClient<T: InitClientConfig>: SendManipulateOperate + SendInstructOperate + SubmoduleOperate {
    async fn init(config: T) -> WrapResult<Self> where Self: Sized + Send + Sync;
    async fn connection_submodule_operate_server(&mut self) -> WrapResult<()>;
    async fn connection_instruct_server(&mut self) -> WrapResult<()>;
    async fn connection_manipulate_server(&mut self) -> WrapResult<()>;
    async fn register(&mut self, operate: ModuleOperate) -> WrapResult<ResponseCode> {
        if self.is_submodule_operate_client_connected() {
            return self.send_register(operate).await;
        }
        Err(NihilityCommonError::NotConnected("Submodule Operate".to_string()))
    }
    async fn heartbeat(&mut self, operate: ModuleOperate) -> WrapResult<ResponseCode> {
        if self.is_submodule_operate_client_connected() {
            return self.send_heartbeat(operate).await;
        }
        Err(NihilityCommonError::NotConnected("Submodule Operate".to_string()))
    }
    async fn offline(&mut self, operate: ModuleOperate) -> WrapResult<ResponseCode> {
        if self.is_submodule_operate_client_connected() {
            return self.send_offline(operate).await;
        }
        Err(NihilityCommonError::NotConnected("Submodule Operate".to_string()))
    }
    async fn update(&mut self, operate: ModuleOperate) -> WrapResult<ResponseCode> {
        if self.is_submodule_operate_client_connected() {
            return self.send_update(operate).await;
        }
        Err(NihilityCommonError::NotConnected("Submodule Operate".to_string()))
    }
    async fn text_instruct(&mut self, instruct: InstructEntity) -> WrapResult<ResponseCode> {
        if self.is_instruct_client_connected() {
            return self.send_text_instruct(instruct).await;
        }
        Err(NihilityCommonError::NotConnected("Instruct".to_string()))
    }
    async fn multiple_text_instruct(
        &mut self,
        instruct_stream: Receiver<InstructEntity>,
    ) -> WrapResult<Receiver<ResponseCode>> {
        if self.is_instruct_client_connected() {
            return self.send_multiple_text_instruct(instruct_stream).await;
        }
        Err(NihilityCommonError::NotConnected("Instruct".to_string()))
    }
    async fn simple_manipulate(&mut self, manipulate: ManipulateEntity) -> WrapResult<ResponseCode> {
        if self.is_manipulate_client_connected() {
            return self.send_simple_manipulate(manipulate).await;
        }
        Err(NihilityCommonError::NotConnected("Manipulate".to_string()))
    }
    async fn text_display_manipulate(&mut self, manipulate: ManipulateEntity) -> WrapResult<ResponseCode> {
        if self.is_manipulate_client_connected() {
            return self.send_text_display_manipulate(manipulate).await;
        }
        Err(NihilityCommonError::NotConnected("Manipulate".to_string()))
    }
    async fn multiple_text_display_manipulate(
        &mut self,
        manipulate_stream: Receiver<ManipulateEntity>,
    ) -> WrapResult<Receiver<ResponseCode>> {
        if self.is_manipulate_client_connected() {
            return self.send_multiple_text_display_manipulate(manipulate_stream).await;
        }
        Err(NihilityCommonError::NotConnected("Manipulate".to_string()))
    }
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

#[async_trait]
pub trait SubmoduleOperate {
    fn is_submodule_operate_client_connected(&self) -> bool;
    async fn send_register(&mut self, operate: ModuleOperate) -> WrapResult<ResponseCode>;
    async fn send_heartbeat(&mut self, operate: ModuleOperate) -> WrapResult<ResponseCode>;
    async fn send_offline(&mut self, operate: ModuleOperate) -> WrapResult<ResponseCode>;
    async fn send_update(&mut self, operate: ModuleOperate) -> WrapResult<ResponseCode>;
}

#[async_trait]
pub trait SendInstructOperate {
    fn is_instruct_client_connected(&self) -> bool;
    async fn send_text_instruct(&mut self, instruct: InstructEntity) -> WrapResult<ResponseCode>;
    async fn send_multiple_text_instruct(
        &mut self,
        instruct_stream: Receiver<InstructEntity>,
    ) -> WrapResult<Receiver<ResponseCode>>;
}

#[async_trait]
pub trait SendManipulateOperate {
    fn is_manipulate_client_connected(&self) -> bool;
    async fn send_simple_manipulate(&mut self, manipulate: ManipulateEntity) -> WrapResult<ResponseCode>;
    async fn send_text_display_manipulate(&mut self, manipulate: ManipulateEntity) -> WrapResult<ResponseCode>;
    async fn send_multiple_text_display_manipulate(
        &mut self,
        manipulate_stream: Receiver<ManipulateEntity>,
    ) -> WrapResult<Receiver<ResponseCode>>;
}
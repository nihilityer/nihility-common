use tokio::sync::mpsc::{Receiver, UnboundedSender};
use tonic::async_trait;

use crate::entity::instruct::InstructEntity;
use crate::entity::manipulate::ManipulateEntity;
use crate::entity::response::ResponseEntity;
use crate::entity::module_operate::ModuleOperate;
use crate::error::{NihilityCommonError, WrapResult};

pub mod grpc;

#[async_trait]
pub trait NihilityClient: SendManipulateOperate + SendInstructOperate + SubmoduleOperate {
    async fn connection_submodule_operate_server(&mut self) -> WrapResult<()>;
    async fn connection_instruct_server(&mut self) -> WrapResult<()>;
    async fn connection_manipulate_server(&mut self) -> WrapResult<()>;
    fn disconnection_submodule_operate_server(&mut self) -> WrapResult<()>;
    fn disconnection_instruct_server(&mut self) -> WrapResult<()>;
    fn disconnection_manipulate_server(&mut self) -> WrapResult<()>;
    async fn register(&self, operate: ModuleOperate) -> WrapResult<ResponseEntity> {
        if self.is_submodule_operate_client_connected() {
            return self.send_register(operate).await;
        }
        Err(NihilityCommonError::NotConnected(
            "Submodule Operate".to_string(),
        ))
    }
    async fn heartbeat(&self, operate: ModuleOperate) -> WrapResult<ResponseEntity> {
        if self.is_submodule_operate_client_connected() {
            return self.send_heartbeat(operate).await;
        }
        Err(NihilityCommonError::NotConnected(
            "Submodule Operate".to_string(),
        ))
    }
    async fn offline(&self, operate: ModuleOperate) -> WrapResult<ResponseEntity> {
        if self.is_submodule_operate_client_connected() {
            return self.send_offline(operate).await;
        }
        Err(NihilityCommonError::NotConnected(
            "Submodule Operate".to_string(),
        ))
    }
    async fn update(&self, operate: ModuleOperate) -> WrapResult<ResponseEntity> {
        if self.is_submodule_operate_client_connected() {
            return self.send_update(operate).await;
        }
        Err(NihilityCommonError::NotConnected(
            "Submodule Operate".to_string(),
        ))
    }
    async fn text_instruct(&self, instruct: InstructEntity) -> WrapResult<ResponseEntity> {
        if self.is_instruct_client_connected() {
            return self.send_text_instruct(instruct).await;
        }
        Err(NihilityCommonError::NotConnected("Instruct".to_string()))
    }
    async fn multiple_text_instruct(
        &self,
        instruct_stream: Receiver<InstructEntity>,
    ) -> WrapResult<Receiver<ResponseEntity>> {
        if self.is_instruct_client_connected() {
            return self.send_multiple_text_instruct(instruct_stream).await;
        }
        Err(NihilityCommonError::NotConnected("Instruct".to_string()))
    }
    async fn simple_manipulate(&self, manipulate: ManipulateEntity) -> WrapResult<ResponseEntity> {
        if self.is_manipulate_client_connected() {
            return self.send_simple_manipulate(manipulate).await;
        }
        Err(NihilityCommonError::NotConnected("Manipulate".to_string()))
    }
    async fn text_display_manipulate(
        &self,
        manipulate: ManipulateEntity,
    ) -> WrapResult<ResponseEntity> {
        if self.is_manipulate_client_connected() {
            return self.send_text_display_manipulate(manipulate).await;
        }
        Err(NihilityCommonError::NotConnected("Manipulate".to_string()))
    }
    async fn multiple_text_display_manipulate(
        &self,
        manipulate_stream: Receiver<ManipulateEntity>,
    ) -> WrapResult<Receiver<ResponseEntity>> {
        if self.is_manipulate_client_connected() {
            return self
                .send_multiple_text_display_manipulate(manipulate_stream)
                .await;
        }
        Err(NihilityCommonError::NotConnected("Manipulate".to_string()))
    }
    async fn direct_connection_manipulate(
        &self,
        manipulate: ManipulateEntity,
    ) -> WrapResult<ResponseEntity> {
        if self.is_manipulate_client_connected() {
            return self.send_direct_connection_manipulate(manipulate).await;
        }
        Err(NihilityCommonError::NotConnected("Manipulate".to_string()))
    }
}

#[async_trait]
pub trait NihilityServer {
    fn set_submodule_operate_sender(
        &mut self,
        submodule_sender: UnboundedSender<ModuleOperate>,
    ) -> WrapResult<()>;

    fn set_instruct_sender(
        &mut self,
        instruct_sender: UnboundedSender<InstructEntity>,
    ) -> WrapResult<()>;

    fn set_manipulate_sender(
        &mut self,
        manipulate_sender: UnboundedSender<ManipulateEntity>,
    ) -> WrapResult<()>;

    fn start(&mut self) -> WrapResult<()>;
}

#[async_trait]
pub trait SubmoduleOperate {
    fn is_submodule_operate_client_connected(&self) -> bool;
    async fn send_register(&self, operate: ModuleOperate) -> WrapResult<ResponseEntity>;
    async fn send_heartbeat(&self, operate: ModuleOperate) -> WrapResult<ResponseEntity>;
    async fn send_offline(&self, operate: ModuleOperate) -> WrapResult<ResponseEntity>;
    async fn send_update(&self, operate: ModuleOperate) -> WrapResult<ResponseEntity>;
}

#[async_trait]
pub trait SendInstructOperate {
    fn is_instruct_client_connected(&self) -> bool;
    async fn send_text_instruct(&self, instruct: InstructEntity) -> WrapResult<ResponseEntity>;
    async fn send_multiple_text_instruct(
        &self,
        instruct_stream: Receiver<InstructEntity>,
    ) -> WrapResult<Receiver<ResponseEntity>>;
}

#[async_trait]
pub trait SendManipulateOperate {
    fn is_manipulate_client_connected(&self) -> bool;
    async fn send_simple_manipulate(
        &self,
        manipulate: ManipulateEntity,
    ) -> WrapResult<ResponseEntity>;
    async fn send_text_display_manipulate(
        &self,
        manipulate: ManipulateEntity,
    ) -> WrapResult<ResponseEntity>;
    async fn send_multiple_text_display_manipulate(
        &self,
        manipulate_stream: Receiver<ManipulateEntity>,
    ) -> WrapResult<Receiver<ResponseEntity>>;
    async fn send_direct_connection_manipulate(
        &self,
        manipulate: ManipulateEntity,
    ) -> WrapResult<ResponseEntity>;
}

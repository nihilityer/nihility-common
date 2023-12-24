use async_trait::async_trait;
use tonic::Request;

use crate::communicat::SubmoduleOperate;
use crate::entity::response::ResponseCode;
use crate::entity::submodule::ModuleOperate;
use crate::error::WrapResult;

use super::GrpcClient;

#[async_trait]
impl SubmoduleOperate for GrpcClient {
    fn is_submodule_operate_client_connected(&self) -> bool {
        if let None = self.submodule_operate_client {
            return false;
        }
        true
    }
    async fn send_register(&self, operate: ModuleOperate) -> WrapResult<ResponseCode> {
        Ok(ResponseCode::from(self.submodule_operate_client.clone().unwrap()
            .register(Request::new(operate.try_into()?))
            .await?
            .into_inner()
            .code()))
    }

    async fn send_heartbeat(&self, operate: ModuleOperate) -> WrapResult<ResponseCode> {
        Ok(ResponseCode::from(self.submodule_operate_client.clone().unwrap()
            .heartbeat(Request::new(operate.try_into()?))
            .await?
            .into_inner()
            .code()))
    }

    async fn send_offline(&self, operate: ModuleOperate) -> WrapResult<ResponseCode> {
        Ok(ResponseCode::from(self.submodule_operate_client.clone().unwrap()
            .offline(Request::new(operate.try_into()?))
            .await?
            .into_inner()
            .code()))
    }

    async fn send_update(&self, operate: ModuleOperate) -> WrapResult<ResponseCode> {
        Ok(ResponseCode::from(self.submodule_operate_client.clone().unwrap()
            .update(Request::new(operate.try_into()?))
            .await?
            .into_inner()
            .code()))
    }
}
use async_trait::async_trait;
use tonic::Request;

use crate::communicat::SubmoduleOperate;
use crate::entity::module_operate::ModuleOperate;
use crate::entity::response::ResponseEntity;
use crate::error::WrapResult;
use crate::utils::auth::submodule_resister_success;
use crate::utils::auth::{get_public_key, signature, verify, Signature};

use super::GrpcClient;

#[async_trait]
impl SubmoduleOperate for GrpcClient {
    fn is_submodule_operate_client_connected(&self) -> bool {
        if self.module_operate_client.is_none() {
            return false;
        }
        true
    }
    async fn send_register(&self, mut operate: ModuleOperate) -> WrapResult<ResponseEntity> {
        let mut buf = [0u8; 512];
        let auth_id = operate.name.to_string();
        signature(
            &mut operate,
            &auth_id,
            get_public_key(&auth_id).await?,
            &mut buf,
        )?;
        let mut resp = ResponseEntity::from(
            self.module_operate_client
                .clone()
                .unwrap()
                .register(Request::new(operate.try_into()?))
                .await?
                .into_inner(),
        );
        if !verify(&mut resp, &mut buf) {
            resp.authentication_fail()
        }
        submodule_resister_success(&mut resp).await?;
        Ok(resp)
    }

    async fn send_heartbeat(&self, mut operate: ModuleOperate) -> WrapResult<ResponseEntity> {
        let mut buf = [0u8; 512];
        let auth_id = String::from_utf8_lossy(operate.get_sign()).to_string();
        signature(
            &mut operate,
            &auth_id,
            get_public_key(&auth_id).await?,
            &mut buf,
        )?;
        let mut resp = ResponseEntity::from(
            self.module_operate_client
                .clone()
                .unwrap()
                .heartbeat(Request::new(operate.try_into()?))
                .await?
                .into_inner(),
        );
        if !verify(&mut resp, &mut buf) {
            resp.authentication_fail()
        }
        Ok(resp)
    }

    async fn send_offline(&self, mut operate: ModuleOperate) -> WrapResult<ResponseEntity> {
        let mut buf = [0u8; 512];
        let auth_id = String::from_utf8_lossy(operate.get_sign()).to_string();
        signature(
            &mut operate,
            &auth_id,
            get_public_key(&auth_id).await?,
            &mut buf,
        )?;
        let mut resp = ResponseEntity::from(
            self.module_operate_client
                .clone()
                .unwrap()
                .offline(Request::new(operate.try_into()?))
                .await?
                .into_inner(),
        );
        if !verify(&mut resp, &mut buf) {
            resp.authentication_fail()
        }
        Ok(resp)
    }

    async fn send_update(&self, mut operate: ModuleOperate) -> WrapResult<ResponseEntity> {
        let mut buf = [0u8; 512];
        let auth_id = String::from_utf8_lossy(operate.get_sign()).to_string();
        signature(
            &mut operate,
            &auth_id,
            get_public_key(&auth_id).await?,
            &mut buf,
        )?;
        let mut resp = ResponseEntity::from(
            self.module_operate_client
                .clone()
                .unwrap()
                .update(Request::new(operate.try_into()?))
                .await?
                .into_inner(),
        );
        if !verify(&mut resp, &mut buf) {
            resp.authentication_fail()
        }
        Ok(resp)
    }
}

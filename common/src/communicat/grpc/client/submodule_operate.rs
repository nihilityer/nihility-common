use async_trait::async_trait;
use tonic::Request;

use crate::communicat::SubmoduleOperate;
use crate::entity::response::ResponseEntity;
use crate::entity::submodule::ModuleOperate;
use crate::error::WrapResult;
use crate::utils::auth::{get_public_key, signature, verify, Signature};

use super::GrpcClient;

#[async_trait]
impl SubmoduleOperate for GrpcClient {
    fn is_submodule_operate_client_connected(&self) -> bool {
        if self.submodule_operate_client.is_none() {
            return false;
        }
        true
    }
    async fn send_register(&self, mut operate: ModuleOperate) -> WrapResult<ResponseEntity> {
        let mut buf = [0u8; 512];
        let auth_id = String::from_utf8_lossy(&operate.get_sign()).to_string();
        let public_key = get_public_key(&auth_id)?;
        signature(&mut operate, &auth_id, public_key, &mut buf)?;
        let mut resp = ResponseEntity::from(
            self.submodule_operate_client
                .clone()
                .unwrap()
                .register(Request::new(operate.try_into()?))
                .await?
                .into_inner(),
        );
        if !verify(&mut resp, &mut buf) {
            resp.authentication_fail()
        }
        Ok(resp)
    }

    async fn send_heartbeat(&self, mut operate: ModuleOperate) -> WrapResult<ResponseEntity> {
        let mut buf = [0u8; 512];
        let auth_id = String::from_utf8_lossy(&operate.get_sign()).to_string();
        let public_key = get_public_key(&auth_id)?;
        signature(&mut operate, &auth_id, public_key, &mut buf)?;
        let mut resp = ResponseEntity::from(
            self.submodule_operate_client
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
        let auth_id = String::from_utf8_lossy(&operate.get_sign()).to_string();
        let public_key = get_public_key(&auth_id)?;
        signature(&mut operate, &auth_id, public_key, &mut buf)?;
        let mut resp = ResponseEntity::from(
            self.submodule_operate_client
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
        let auth_id = String::from_utf8_lossy(&operate.get_sign()).to_string();
        let public_key = get_public_key(&auth_id)?;
        signature(&mut operate, &auth_id, public_key, &mut buf)?;
        let mut resp = ResponseEntity::from(
            self.submodule_operate_client
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

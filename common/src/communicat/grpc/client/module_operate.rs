use async_trait::async_trait;
use rsa::pkcs8::{EncodePublicKey, LineEnding};
use tonic::Request;

use crate::communicat::SubmoduleOperate;
use crate::entity::module_operate::ModuleOperate;
use crate::entity::response::ResponseEntity;
use crate::error::WrapResult;
use crate::utils::auth::submodule_resister_success;
use crate::utils::auth::{get_public_key, signature, verify, Signature, SUBMODULE_PUBLIC_KEY};
use crate::{get_submodule_name, submodule_authentication_core_init, OperateType, SubmoduleInfo};

use super::GrpcClient;

#[async_trait]
impl SubmoduleOperate for GrpcClient {
    fn is_submodule_operate_client_connected(&self) -> bool {
        if self.module_operate_client.is_none() {
            return false;
        }
        true
    }
    async fn send_register(&self, mut submodule_info: SubmoduleInfo) -> WrapResult<ResponseEntity> {
        let mut buf = [0u8; 512];
        let mut operate = ModuleOperate::default();

        operate.name = get_submodule_name();
        submodule_info.conn_params.conn_config.insert(
            SUBMODULE_PUBLIC_KEY.to_string(),
            submodule_authentication_core_init()?.to_public_key_pem(LineEnding::default())?,
        );
        operate.info = Some(submodule_info);
        operate.operate_type = OperateType::Register;
        signature(
            &mut operate,
            &get_submodule_name(),
            get_public_key(&get_submodule_name()).await?,
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
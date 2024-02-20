use async_trait::async_trait;
use rsa::pkcs8::{EncodePublicKey, LineEnding};
use tokio::{select, spawn};
use tokio_util::sync::CancellationToken;
use tonic::Request;
use tracing::{error, info};

use crate::communicat::{heartbeat_thread, SubmoduleOperate};
use crate::entity::module_operate::ModuleOperate;
use crate::entity::response::ResponseEntity;
use crate::error::{NihilityCommonError, WrapResult};
use crate::utils::auth::{get_public_key, signature, verify, Signature, SUBMODULE_PUBLIC_KEY};
use crate::utils::auth::{submodule_authentication_core_init, submodule_resister_success};
use crate::{get_submodule_name, OperateType, SubmoduleInfo};

use super::GrpcClient;

#[async_trait]
impl SubmoduleOperate for GrpcClient {
    fn is_submodule_operate_client_connected(&self) -> bool {
        self.module_operate_client.is_some()
    }
    async fn send_register(
        &mut self,
        mut submodule_info: SubmoduleInfo,
    ) -> WrapResult<ResponseEntity> {
        let mut buf = [0u8; 512];
        let mut operate = ModuleOperate::default();
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

    async fn send_heartbeat(&self) -> WrapResult<ResponseEntity> {
        let mut buf = [0u8; 512];
        let mut operate = ModuleOperate::default();
        let auth_id = String::from_utf8_lossy(operate.get_sign()).to_string();
        operate.operate_type = OperateType::Heartbeat;
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

    async fn send_offline(&mut self, submodule_info: SubmoduleInfo) -> WrapResult<ResponseEntity> {
        let mut buf = [0u8; 512];
        let mut operate = ModuleOperate::default();
        let auth_id = String::from_utf8_lossy(operate.get_sign()).to_string();
        operate.operate_type = OperateType::Offline;
        operate.info = Some(submodule_info);
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

    async fn send_update(&self, submodule_info: SubmoduleInfo) -> WrapResult<ResponseEntity> {
        let mut buf = [0u8; 512];
        let mut operate = ModuleOperate::default();
        operate.operate_type = OperateType::Update;
        operate.info = Some(submodule_info);
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
    async fn start_heartbeat_thread(&mut self) -> WrapResult<()> {
        let cancellation_token = CancellationToken::new();
        let thread_cancellation_token = cancellation_token.clone();
        let client = self.clone();
        spawn(async move {
            select! {
                heartbeat_thread_result = heartbeat_thread(client) => {
                    if let Err(e) = heartbeat_thread_result {
                        error!("Heartbeat Thread Error: {}", e);
                        thread_cancellation_token.cancel();
                    }
                },
                _ = thread_cancellation_token.cancelled() => {},
            }
        });
        self.cancellation_token = Some(cancellation_token);
        Ok(())
    }

    async fn stop_heartbeat_thread(&mut self) -> WrapResult<()> {
        match &self.cancellation_token {
            None => Err(NihilityCommonError::ThreadNotStarted(String::from(
                "Heartbeat",
            ))),
            Some(cancellation_token) => {
                cancellation_token.cancel();
                self.cancellation_token = None;
                info!("Heartbeat Thread Stopped");
                Ok(())
            }
        }
    }
}

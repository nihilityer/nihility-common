use async_trait::async_trait;
use tokio::spawn;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;
use tonic::Request;
use tracing::error;

use crate::communicat::SendManipulateOperate;
use crate::entity::manipulate::ManipulateEntity;
use crate::entity::response::ResponseEntity;
use crate::error::WrapResult;
use crate::manipulate::TextDisplayManipulate;
use crate::utils::auth::{get_public_key, signature, verify, Signature};

use super::GrpcClient;

const STREAM_BUFFER: usize = 12;

#[async_trait]
impl SendManipulateOperate for GrpcClient {
    fn is_manipulate_client_connected(&self) -> bool {
        if self.manipulate_client.is_none() {
            return false;
        }
        true
    }
    async fn send_simple_manipulate(
        &self,
        mut manipulate: ManipulateEntity,
    ) -> WrapResult<ResponseEntity> {
        let mut buf = [0u8; 512];
        let auth_id = String::from_utf8_lossy(manipulate.get_sign()).to_string();
        signature(
            &mut manipulate,
            &auth_id,
            get_public_key(&auth_id).await?,
            &mut buf,
        )?;
        let mut resp = ResponseEntity::from(
            self.manipulate_client
                .clone()
                .unwrap()
                .send_simple_manipulate(Request::new(manipulate.try_into()?))
                .await?
                .into_inner(),
        );
        if !verify(&mut resp, &mut buf) {
            resp.authentication_fail()
        }
        Ok(resp)
    }

    async fn send_text_display_manipulate(
        &self,
        mut manipulate: ManipulateEntity,
    ) -> WrapResult<ResponseEntity> {
        let mut buf = [0u8; 512];
        let auth_id = String::from_utf8_lossy(manipulate.get_sign()).to_string();
        signature(
            &mut manipulate,
            &auth_id,
            get_public_key(&auth_id).await?,
            &mut buf,
        )?;
        let mut resp = ResponseEntity::from(
            self.manipulate_client
                .clone()
                .unwrap()
                .send_text_display_manipulate(Request::new(manipulate.try_into()?))
                .await?
                .into_inner(),
        );
        if !verify(&mut resp, &mut buf) {
            resp.authentication_fail()
        }
        Ok(resp)
    }

    async fn send_multiple_text_display_manipulate(
        &self,
        mut manipulate_stream: Receiver<ManipulateEntity>,
    ) -> WrapResult<Receiver<ResponseEntity>> {
        let (req_tx, req_rx) = mpsc::channel::<TextDisplayManipulate>(STREAM_BUFFER);
        let (out_tx, out_rx) = mpsc::channel::<ResponseEntity>(STREAM_BUFFER);
        spawn(async move {
            while let Some(mut manipulate) = manipulate_stream.recv().await {
                let mut buf = [0u8; 512];
                let auth_id = String::from_utf8_lossy(manipulate.get_sign()).to_string();
                match get_public_key(&auth_id).await {
                    Ok(public_key) => {
                        signature(&mut manipulate, &auth_id, public_key, &mut buf)
                            .expect("Encode Entity Error");
                        match <ManipulateEntity as TryInto<TextDisplayManipulate>>::try_into(
                            manipulate,
                        ) {
                            Ok(text_display_manipulate) => {
                                match req_tx.send(text_display_manipulate).await {
                                    Ok(_) => {}
                                    Err(e) => {
                                        error!("Grpc Client send_multiple_text_display_manipulate Send To Stream Error: {:?}", e);
                                        break;
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Grpc Client send_multiple_text_display_manipulate Transform Error: {:?}", e);
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        error!(
                            "Grpc Manipulate Server send_multiple_text_display_manipulate Auth Id Error: {:?}",
                            &e
                        );
                        break;
                    }
                }
            }
        });
        let mut resp_stream = self
            .manipulate_client
            .clone()
            .unwrap()
            .send_multiple_text_display_manipulate(ReceiverStream::new(req_rx))
            .await?
            .into_inner();
        spawn(async move {
            let mut buf = [0u8; 512];
            while let Some(result) = resp_stream.next().await {
                match result {
                    Ok(resp) => {
                        let mut entity = ResponseEntity::from(resp);
                        if !verify(&mut entity, &mut buf) {
                            entity.authentication_fail()
                        }
                        match out_tx.send(entity).await {
                            Ok(_) => {}
                            Err(e) => {
                                error!("Manipulate Grpc Client send_multiple_text_display_manipulate Send To Core Error: {:?}", e);
                                break;
                            }
                        }
                    }
                    Err(status) => {
                        error!(
                            "Manipulate Grpc Client send_multiple_text_display_manipulate Send Error: {:?}",
                            &status
                        );
                        let mut resp = ResponseEntity::default();
                        resp.unknown_error();
                        match out_tx.send(resp).await {
                            Ok(_) => {}
                            Err(e) => {
                                error!("Manipulate Grpc Client send_multiple_text_display_manipulate Send To Core Error: {:?}", e);
                                break;
                            }
                        }
                    }
                }
            }
        });
        Ok(out_rx)
    }

    async fn send_direct_connection_manipulate(
        &self,
        mut manipulate: ManipulateEntity,
    ) -> WrapResult<ResponseEntity> {
        let mut buf = [0u8; 512];
        let auth_id = String::from_utf8_lossy(&manipulate.get_sign()).to_string();
        signature(
            &mut manipulate,
            &auth_id,
            get_public_key(&auth_id).await?,
            &mut buf,
        )?;
        let mut resp = ResponseEntity::from(
            self.manipulate_client
                .clone()
                .unwrap()
                .send_direct_connection_manipulate(Request::new(manipulate.try_into()?))
                .await?
                .into_inner(),
        );
        if !verify(&mut resp, &mut buf) {
            resp.authentication_fail()
        }
        Ok(resp)
    }
}

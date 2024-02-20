use async_trait::async_trait;
use tokio::spawn;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;
use tonic::Request;
use tracing::error;

use crate::communicat::SendInstructOperate;
use crate::entity::instruct::InstructEntity;
use crate::entity::response::ResponseEntity;
use crate::error::WrapResult;
use crate::instruct::TextInstruct;
use crate::utils::auth::{get_public_key, signature, verify, Signature};

use super::GrpcClient;

const STREAM_BUFFER: usize = 12;

#[async_trait]
impl SendInstructOperate for GrpcClient {
    fn is_instruct_client_connected(&self) -> bool {
        self.instruct_client.is_some()
    }

    async fn send_text_instruct(&self, mut instruct: InstructEntity) -> WrapResult<ResponseEntity> {
        let mut buf = [0u8; 512];
        let auth_id = String::from_utf8_lossy(instruct.get_sign()).to_string();
        signature(
            &mut instruct,
            &auth_id,
            get_public_key(&auth_id).await?,
            &mut buf,
        )?;
        let mut resp = ResponseEntity::from(
            self.instruct_client
                .clone()
                .unwrap()
                .send_text_instruct(Request::new(instruct.try_into()?))
                .await?
                .into_inner(),
        );
        if !verify(&mut resp, &mut buf) {
            resp.authentication_fail()
        }
        Ok(resp)
    }

    async fn send_multiple_text_instruct(
        &self,
        mut instruct_stream: Receiver<InstructEntity>,
    ) -> WrapResult<Receiver<ResponseEntity>> {
        let (req_tx, req_rx) = mpsc::channel::<TextInstruct>(STREAM_BUFFER);
        let (out_tx, out_rx) = mpsc::channel::<ResponseEntity>(STREAM_BUFFER);
        spawn(async move {
            let mut buf = [0u8; 512];
            while let Some(mut instruct) = instruct_stream.recv().await {
                let auth_id = String::from_utf8_lossy(instruct.get_sign()).to_string();
                match get_public_key(&auth_id).await {
                    Ok(public_key) => {
                        signature(&mut instruct, &auth_id, public_key, &mut buf)
                            .expect("Encode Entity Error");
                        match <InstructEntity as TryInto<TextInstruct>>::try_into(instruct) {
                            Ok(text_instruct) => match req_tx.send(text_instruct).await {
                                Ok(_) => {}
                                Err(e) => {
                                    error!("Grpc Client send_multiple_text_instruct Send To Stream Error: {:?}", e);
                                    break;
                                }
                            },
                            Err(e) => {
                                error!(
                                    "Grpc Client send_multiple_text_instruct Transform Error: {:?}",
                                    e
                                );
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        error!(
                            "Grpc Instruct Server send_text_instruct Auth Id Error: {:?}",
                            &e
                        );
                        break;
                    }
                }
            }
        });
        let mut resp_stream = self
            .instruct_client
            .clone()
            .unwrap()
            .send_multiple_text_instruct(ReceiverStream::new(req_rx))
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
                                error!("Instruct Grpc Client send_multiple_text_instruct Send To Core Error: {:?}", e);
                                break;
                            }
                        }
                    }
                    Err(status) => {
                        error!(
                            "Instruct Grpc Client send_multiple_text_instruct Send Error: {:?}",
                            &status
                        );
                        let mut resp = ResponseEntity::default();
                        resp.unknown_error();
                        match out_tx.send(resp).await {
                            Ok(_) => {}
                            Err(e) => {
                                error!("Instruct Grpc Client send_multiple_text_instruct Send To Core Error: {:?}", e);
                                break;
                            }
                        }
                    }
                }
            }
        });
        Ok(out_rx)
    }
}

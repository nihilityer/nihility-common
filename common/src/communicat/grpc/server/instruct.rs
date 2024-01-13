use tokio::spawn;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use tokio_stream::wrappers::ReceiverStream;
use tonic::codegen::tokio_stream::StreamExt;
use tonic::{Code, Request, Response, Status, Streaming};
use tracing::error;

use crate::communicat::grpc::server::StreamResp;
use crate::entity::instruct::InstructEntity;
use crate::entity::response::ResponseEntity;
use crate::instruct::instruct_server::Instruct;
use crate::instruct::TextInstruct;
use crate::response_code::Resp;
use crate::utils::auth::{
    get_public_key, signature, verify, Signature, AUTHENTICATION_ERROR_MESSAGE,
};

#[derive(Clone)]
pub struct InstructImpl {
    instruct_sender: UnboundedSender<InstructEntity>,
}

impl InstructImpl {
    pub fn init(sender: UnboundedSender<InstructEntity>) -> Self {
        InstructImpl {
            instruct_sender: sender,
        }
    }
}

#[tonic::async_trait]
impl Instruct for InstructImpl {
    async fn send_text_instruct(
        &self,
        request: Request<TextInstruct>,
    ) -> Result<Response<Resp>, Status> {
        let mut buf = [0u8; 512];
        let mut entity = InstructEntity::from(request.into_inner());
        if verify(&mut entity, &mut buf) {
            let auth_id = String::from_utf8_lossy(entity.get_sign()).to_string();
            match self.instruct_sender.send(entity) {
                Ok(_) => match get_public_key(&auth_id) {
                    Ok(public_key) => {
                        let mut resp = ResponseEntity::default();
                        signature(&mut resp, &auth_id, public_key, &mut buf)
                            .expect("Encode Entity Error");
                        Ok(Response::new(Resp::from(resp)))
                    }
                    Err(e) => {
                        error!(
                            "Grpc Instruct Server send_text_instruct Auth Id Error: {:?}",
                            &e
                        );
                        Err(Status::from_error(Box::new(e)))
                    }
                },
                Err(e) => {
                    error!(
                        "Grpc Instruct Server send_text_instruct Auth Id Error: {:?}",
                        &e
                    );
                    Err(Status::from_error(Box::new(e)))
                }
            }
        } else {
            Err(Status::new(Code::Ok, AUTHENTICATION_ERROR_MESSAGE))
        }
    }

    type SendMultipleTextInstructStream = StreamResp;

    async fn send_multiple_text_instruct(
        &self,
        request: Request<Streaming<TextInstruct>>,
    ) -> Result<Response<Self::SendMultipleTextInstructStream>, Status> {
        let mut req_stream = request.into_inner();
        let (tx, rx) = mpsc::channel(128);
        let instruct_sender = self.instruct_sender.clone();
        spawn(async move {
            let mut buf = [0u8; 512];
            while let Some(result) = req_stream.next().await {
                match result {
                    Ok(instruct) => {
                        let mut entity = InstructEntity::from(instruct);
                        if verify(&mut entity, &mut buf) {
                            let auth_id = String::from_utf8_lossy(entity.get_sign()).to_string();
                            match instruct_sender.send(entity) {
                                Ok(_) => {
                                    let mut resp = ResponseEntity::default();
                                    match get_public_key(&auth_id) {
                                        Ok(public_key) => {
                                            signature(&mut resp, &auth_id, public_key, &mut buf)
                                                .expect("Encode Entity Error");
                                            match tx.send(Ok(Resp::from(resp))).await {
                                                Ok(_) => {}
                                                Err(e) => {
                                                    error!("Manipulate Server send_multiple_text_display_manipulate Send To Stream Error: {:?}", e);
                                                    break;
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            error!(
                                                "Manipulate Server send_multiple_text_display_manipulate Auth Id Error: {:?}",
                                                &e
                                            );
                                            match tx
                                                .send(Err(Status::from_error(Box::new(e))))
                                                .await
                                            {
                                                Ok(_) => {}
                                                Err(e) => {
                                                    error!("Manipulate Server send_multiple_text_display_manipulate Send To Stream Error: {:?}", e);
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    error!("Manipulate Server send_multiple_text_display_manipulate Send To Core Error: {:?}", e);
                                    match get_public_key(&auth_id) {
                                        Ok(public_key) => {
                                            let mut resp = ResponseEntity::default();
                                            resp.unknown_error();
                                            signature(&mut resp, &auth_id, public_key, &mut buf)
                                                .expect("Encode Entity Error");
                                            match tx.send(Ok(Resp::from(resp))).await {
                                                Ok(_) => {}
                                                Err(e) => {
                                                    error!("Manipulate Server send_multiple_text_display_manipulate Send To Stream Error: {:?}", e);
                                                    break;
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            error!(
                                                "Manipulate Server send_multiple_text_display_manipulate Auth Id Error: {:?}",
                                                &e
                                            );
                                            match tx
                                                .send(Err(Status::from_error(Box::new(e))))
                                                .await
                                            {
                                                Ok(_) => {}
                                                Err(e) => {
                                                    error!("Manipulate Server send_multiple_text_display_manipulate Send To Stream Error: {:?}", e);
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            error!(
                                "Instruct Server send_multiple_text_instruct Authentication Fail"
                            );
                            match tx
                                .send(Err(Status::new(Code::Ok, AUTHENTICATION_ERROR_MESSAGE)))
                                .await
                            {
                                Ok(_) => {}
                                Err(e) => {
                                    error!("Instruct Server send_multiple_text_instruct Send To Stream Error: {:?}", e);
                                    break;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!(
                            "Instruct Server send_multiple_text_instruct Receive Error: {:?}",
                            &e
                        );
                        match tx.send(Err(e)).await {
                            Ok(_) => {}
                            Err(e) => {
                                error!("Instruct Server send_multiple_text_instruct Send To Stream Error: {:?}", e);
                                break;
                            }
                        }
                    }
                }
            }
        });
        Ok(Response::new(
            Box::pin(ReceiverStream::new(rx)) as Self::SendMultipleTextInstructStream
        ))
    }
}

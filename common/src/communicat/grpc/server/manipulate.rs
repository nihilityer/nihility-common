use tokio::spawn;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;
use tonic::{Code, Request, Response, Status, Streaming};
use tracing::error;

use crate::communicat::grpc::server::StreamResp;
use crate::entity::manipulate::ManipulateEntity;
use crate::entity::response::ResponseEntity;
use crate::manipulate::manipulate_server::Manipulate;
use crate::manipulate::{DirectConnectionManipulate, SimpleManipulate, TextDisplayManipulate};
use crate::response_code::Resp;
use crate::utils::auth::{
    get_public_key, signature, verify, Signature, AUTHENTICATION_ERROR_MESSAGE,
};

#[derive(Clone)]
pub struct ManipulateImpl {
    manipulate_sender: UnboundedSender<ManipulateEntity>,
}

#[tonic::async_trait]
impl Manipulate for ManipulateImpl {
    async fn send_simple_manipulate(
        &self,
        request: Request<SimpleManipulate>,
    ) -> Result<Response<Resp>, Status> {
        let mut buf = [0u8; 512];
        let mut entity = ManipulateEntity::from(request.into_inner());
        if verify(&mut entity, &mut buf) {
            let auth_id = String::from_utf8_lossy(entity.get_sign()).to_string();
            match self.manipulate_sender.send(entity) {
                Ok(_) => match get_public_key(&auth_id).await {
                    Ok(public_key) => {
                        let mut resp = ResponseEntity::default();
                        signature(&mut resp, &auth_id, public_key, &mut buf)
                            .expect("Encode Entity Error");
                        Ok(Response::new(Resp::from(resp)))
                    }
                    Err(e) => {
                        error!(
                            "Grpc Manipulate Server send_simple_manipulate Error: {:?}",
                            &e
                        );
                        Err(Status::from_error(Box::new(e)))
                    }
                },
                Err(e) => {
                    error!(
                        "Grpc Manipulate Server send_simple_manipulate Auth Id Error: {:?}",
                        &e
                    );
                    Err(Status::from_error(Box::new(e)))
                }
            }
        } else {
            Err(Status::new(Code::Ok, AUTHENTICATION_ERROR_MESSAGE))
        }
    }

    async fn send_text_display_manipulate(
        &self,
        request: Request<TextDisplayManipulate>,
    ) -> Result<Response<Resp>, Status> {
        let mut buf = [0u8; 512];
        let mut entity = ManipulateEntity::from(request.into_inner());
        if verify(&mut entity, &mut buf) {
            let auth_id = String::from_utf8_lossy(entity.get_sign()).to_string();
            match self.manipulate_sender.send(entity) {
                Ok(_) => match get_public_key(&auth_id).await {
                    Ok(public_key) => {
                        let mut resp = ResponseEntity::default();
                        signature(&mut resp, &auth_id, public_key, &mut buf)
                            .expect("Encode Entity Error");
                        Ok(Response::new(Resp::from(resp)))
                    }
                    Err(e) => {
                        error!(
                            "Grpc Manipulate Server send_text_display_manipulate Error: {:?}",
                            &e
                        );
                        Err(Status::from_error(Box::new(e)))
                    }
                },
                Err(e) => {
                    error!(
                        "Grpc Manipulate Server send_text_display_manipulate Auth Id Error: {:?}",
                        &e
                    );
                    Err(Status::from_error(Box::new(e)))
                }
            }
        } else {
            Err(Status::new(Code::Ok, AUTHENTICATION_ERROR_MESSAGE))
        }
    }

    type SendMultipleTextDisplayManipulateStream = StreamResp;

    async fn send_multiple_text_display_manipulate(
        &self,
        request: Request<Streaming<TextDisplayManipulate>>,
    ) -> Result<Response<Self::SendMultipleTextDisplayManipulateStream>, Status> {
        let mut req_stream = request.into_inner();
        let (tx, rx) = mpsc::channel(128);
        let manipulate_sender = self.manipulate_sender.clone();
        spawn(async move {
            let mut buf = [0u8; 512];
            while let Some(result) = req_stream.next().await {
                match result {
                    Ok(manipulate) => {
                        let mut entity = ManipulateEntity::from(manipulate);
                        if verify(&mut entity, &mut buf) {
                            let auth_id = String::from_utf8_lossy(entity.get_sign()).to_string();
                            match manipulate_sender.send(entity) {
                                Ok(_) => {
                                    let mut resp = ResponseEntity::default();
                                    match get_public_key(&auth_id).await {
                                        Ok(public_key) => {
                                            signature(&mut resp, &auth_id, public_key, &mut buf)
                                                .expect("Encode Entity Error");
                                            match tx.send(Ok(Resp::from(resp))).await {
                                                Ok(_) => {}
                                                Err(e) => {
                                                    error!("Instruct Server send_multiple_text_instruct Send To Stream Error: {:?}", e);
                                                    break;
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            error!(
                                                "Instruct Server send_multiple_text_instruct Auth Id Error: {:?}",
                                                &e
                                            );
                                            match tx
                                                .send(Err(Status::from_error(Box::new(e))))
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
                                }
                                Err(e) => {
                                    error!("Instruct Server send_multiple_text_instruct Send To Core Error: {:?}", e);
                                    match get_public_key(&auth_id).await {
                                        Ok(public_key) => {
                                            let mut resp = ResponseEntity::default();
                                            resp.unknown_error();
                                            signature(&mut resp, &auth_id, public_key, &mut buf)
                                                .expect("Encode Entity Error");
                                            match tx.send(Ok(Resp::from(resp))).await {
                                                Ok(_) => {}
                                                Err(e) => {
                                                    error!("Instruct Server send_multiple_text_instruct Send To Stream Error: {:?}", e);
                                                    break;
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            error!(
                                                "Instruct Server send_multiple_text_instruct Auth Id Error: {:?}",
                                                &e
                                            );
                                            match tx
                                                .send(Err(Status::from_error(Box::new(e))))
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
                            "Manipulate Server send_multiple_text_display_manipulate Receive Error: {:?}",
                            &e
                        );
                        match tx.send(Err(e)).await {
                            Ok(_) => {}
                            Err(e) => {
                                error!("Manipulate Server send_multiple_text_display_manipulate Send To Stream Error: {:?}", e);
                                break;
                            }
                        }
                    }
                }
            }
        });
        Ok(Response::new(
            Box::pin(ReceiverStream::new(rx)) as Self::SendMultipleTextDisplayManipulateStream
        ))
    }

    async fn send_direct_connection_manipulate(
        &self,
        request: Request<DirectConnectionManipulate>,
    ) -> Result<Response<Resp>, Status> {
        match ManipulateEntity::try_from(request.into_inner()) {
            Ok(mut entity) => {
                let mut buf = [0u8; 512];
                if verify(&mut entity, &mut buf) {
                    let auth_id = String::from_utf8_lossy(entity.get_sign()).to_string();
                    match self.manipulate_sender.send(entity) {
                        Ok(_) => match get_public_key(&auth_id).await {
                            Ok(public_key) => {
                                let mut resp = ResponseEntity::default();
                                signature(&mut resp, &auth_id, public_key, &mut buf)
                                    .expect("Encode Entity Error");
                                Ok(Response::new(Resp::from(resp)))
                            }
                            Err(e) => {
                                error!(
                    "Grpc Manipulate Server send_direct_connection_manipulate Error: {:?}",
                    &e
                );
                                Err(Status::from_error(Box::new(e)))
                            }
                        },
                        Err(e) => {
                            error!(
                        "Grpc Manipulate Server send_direct_connection_manipulate Auth Id Error: {:?}",
                        &e
                    );
                            Err(Status::from_error(Box::new(e)))
                        }
                    }
                } else {
                    Err(Status::new(Code::Ok, AUTHENTICATION_ERROR_MESSAGE))
                }
            }
            Err(e) => {
                error!(
                    "Grpc Manipulate Server send_direct_connection_manipulate Error: {:?}",
                    &e
                );
                Err(Status::from_error(Box::new(e)))
            }
        }
    }
}

impl ManipulateImpl {
    pub fn init(sender: UnboundedSender<ManipulateEntity>) -> Self {
        ManipulateImpl {
            manipulate_sender: sender,
        }
    }
}

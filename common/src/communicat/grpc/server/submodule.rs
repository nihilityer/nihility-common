use tokio::sync::mpsc::UnboundedSender;
use tonic::{Code, Request, Response, Status};
use tracing::error;

use crate::entity::response::ResponseEntity;
use crate::entity::submodule::{ModuleOperate, OperateType};
use crate::response_code::Resp;
use crate::submodule::submodule_server::Submodule;
use crate::submodule::{SubmoduleHeartbeat, SubmoduleReq};
use crate::utils::auth::{
    get_public_key, signature, verify, Signature, AUTHENTICATION_ERROR_MESSAGE,
};

#[derive(Clone)]
pub struct SubmoduleImpl {
    operate_module_sender: UnboundedSender<ModuleOperate>,
}

impl SubmoduleImpl {
    pub fn init(operate_module_sender: UnboundedSender<ModuleOperate>) -> Self {
        SubmoduleImpl {
            operate_module_sender,
        }
    }
}

#[tonic::async_trait]
impl Submodule for SubmoduleImpl {
    async fn register(&self, request: Request<SubmoduleReq>) -> Result<Response<Resp>, Status> {
        let mut buf = [0u8; 512];
        match ModuleOperate::try_from(request.into_inner()) {
            Ok(mut operate) => {
                operate.operate_type = OperateType::Register;
                if verify(&mut operate, &mut buf) {
                    let auth_id = String::from_utf8_lossy(operate.get_sign()).to_string();
                    match self.operate_module_sender.send(operate) {
                        Ok(_) => match get_public_key(&auth_id) {
                            Ok(public_key) => {
                                let mut resp = ResponseEntity::default();
                                signature(&mut resp, &auth_id, public_key, &mut buf)
                                    .expect("Encode Entity Error");
                                Ok(Response::new(Resp::from(resp)))
                            }
                            Err(e) => {
                                error!("Submodule Server register Auth Id Error: {:?}", &e);
                                Err(Status::from_error(Box::new(e)))
                            }
                        },
                        Err(e) => {
                            error!("Submodule Server register Auth Id Error: {:?}", &e);
                            Err(Status::from_error(Box::new(e)))
                        }
                    }
                } else {
                    Err(Status::new(Code::Ok, AUTHENTICATION_ERROR_MESSAGE))
                }
            }
            Err(e) => {
                error!(
                    "Submodule Server register Create Operate From req Error: {:?}",
                    &e
                );
                Err(Status::from_error(Box::new(e)))
            }
        }
    }

    async fn offline(&self, request: Request<SubmoduleReq>) -> Result<Response<Resp>, Status> {
        let mut buf = [0u8; 512];
        match ModuleOperate::try_from(request.into_inner()) {
            Ok(mut operate) => {
                operate.operate_type = OperateType::Offline;
                if verify(&mut operate, &mut buf) {
                    let auth_id = String::from_utf8_lossy(operate.get_sign()).to_string();
                    match self.operate_module_sender.send(operate) {
                        Ok(_) => match get_public_key(&auth_id) {
                            Ok(public_key) => {
                                let mut resp = ResponseEntity::default();
                                signature(&mut resp, &auth_id, public_key, &mut buf)
                                    .expect("Encode Entity Error");
                                Ok(Response::new(Resp::from(resp)))
                            }
                            Err(e) => {
                                error!("Submodule Server offline Auth Id Error: {:?}", &e);
                                Err(Status::from_error(Box::new(e)))
                            }
                        },
                        Err(e) => {
                            error!("Submodule Server offline Auth Id Error: {:?}", &e);
                            Err(Status::from_error(Box::new(e)))
                        }
                    }
                } else {
                    Err(Status::new(Code::Ok, AUTHENTICATION_ERROR_MESSAGE))
                }
            }
            Err(e) => {
                error!(
                    "Submodule Server offline Create Operate From req Error: {:?}",
                    &e
                );
                Err(Status::from_error(Box::new(e)))
            }
        }
    }

    async fn heartbeat(
        &self,
        request: Request<SubmoduleHeartbeat>,
    ) -> Result<Response<Resp>, Status> {
        let mut buf = [0u8; 512];
        let mut entity = ModuleOperate::from(request.into_inner());
        if verify(&mut entity, &mut buf) {
            let auth_id = String::from_utf8_lossy(entity.get_sign()).to_string();
            match self.operate_module_sender.send(entity) {
                Ok(_) => match get_public_key(&auth_id) {
                    Ok(public_key) => {
                        let mut resp = ResponseEntity::default();
                        signature(&mut resp, &auth_id, public_key, &mut buf)
                            .expect("Encode Entity Error");
                        Ok(Response::new(Resp::from(resp)))
                    }
                    Err(e) => {
                        error!("Submodule Server heartbeat Auth Id Error: {:?}", &e);
                        Err(Status::from_error(Box::new(e)))
                    }
                },
                Err(e) => {
                    error!("Submodule Server heartbeat Auth Id Error: {:?}", &e);
                    Err(Status::from_error(Box::new(e)))
                }
            }
        } else {
            Err(Status::new(Code::Ok, AUTHENTICATION_ERROR_MESSAGE))
        }
    }

    async fn update(&self, request: Request<SubmoduleReq>) -> Result<Response<Resp>, Status> {
        let mut buf = [0u8; 512];
        match ModuleOperate::try_from(request.into_inner()) {
            Ok(mut operate) => {
                operate.operate_type = OperateType::Update;
                if verify(&mut operate, &mut buf) {
                    let auth_id = String::from_utf8_lossy(operate.get_sign()).to_string();
                    match self.operate_module_sender.send(operate) {
                        Ok(_) => match get_public_key(&auth_id) {
                            Ok(public_key) => {
                                let mut resp = ResponseEntity::default();
                                signature(&mut resp, &auth_id, public_key, &mut buf)
                                    .expect("Encode Entity Error");
                                Ok(Response::new(Resp::from(resp)))
                            }
                            Err(e) => {
                                error!("Submodule Server update Auth Id Error: {:?}", &e);
                                Err(Status::from_error(Box::new(e)))
                            }
                        },
                        Err(e) => {
                            error!("Submodule Server update Auth Id Error: {:?}", &e);
                            Err(Status::from_error(Box::new(e)))
                        }
                    }
                } else {
                    Err(Status::new(Code::Ok, AUTHENTICATION_ERROR_MESSAGE))
                }
            }
            Err(e) => {
                error!(
                    "Submodule Server update Create Operate From req Error: {:?}",
                    &e
                );
                Err(Status::from_error(Box::new(e)))
            }
        }
    }
}

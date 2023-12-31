use tokio::sync::mpsc::UnboundedSender;
use tonic::{Request, Response, Status};
use tracing::error;

use crate::entity::submodule::{ModuleOperate, OperateType};
use crate::response_code::{Resp, RespCode};
use crate::submodule::submodule_server::Submodule;
use crate::submodule::{SubmoduleHeartbeat, SubmoduleReq};

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
        match ModuleOperate::try_from(request.into_inner()) {
            Ok(mut operate) => {
                operate.operate_type = OperateType::Register;
                match self.operate_module_sender.send(operate) {
                    Ok(_) => Ok(Response::new(Resp {
                        code: RespCode::Success.into(),
                    })),
                    Err(e) => {
                        error!("Submodule Server register Operate Send Error: {:?}", &e);
                        Err(Status::from_error(Box::new(e)))
                    }
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
        match ModuleOperate::try_from(request.into_inner()) {
            Ok(mut operate) => {
                operate.operate_type = OperateType::Offline;
                match self.operate_module_sender.send(operate) {
                    Ok(_) => Ok(Response::new(Resp {
                        code: RespCode::Success.into(),
                    })),
                    Err(e) => {
                        error!("Submodule Server offline Operate Send Error: {:?}", &e);
                        Err(Status::from_error(Box::new(e)))
                    }
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
        match self
            .operate_module_sender
            .send(ModuleOperate::from(request.into_inner()))
        {
            Ok(_) => Ok(Response::new(Resp {
                code: RespCode::Success.into(),
            })),
            Err(e) => {
                error!("Submodule Server heartbeat Operate Send Error: {:?}", &e);
                Err(Status::from_error(Box::new(e)))
            }
        }
    }

    async fn update(&self, request: Request<SubmoduleReq>) -> Result<Response<Resp>, Status> {
        match ModuleOperate::try_from(request.into_inner()) {
            Ok(mut operate) => {
                operate.operate_type = OperateType::Update;
                match self.operate_module_sender.send(operate) {
                    Ok(_) => Ok(Response::new(Resp {
                        code: RespCode::Success.into(),
                    })),
                    Err(e) => {
                        error!("Submodule Server update Operate Send Error: {:?}", &e);
                        Err(Status::from_error(Box::new(e)))
                    }
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

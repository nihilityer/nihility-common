use tokio::sync::mpsc::UnboundedSender;
use tonic::{Request, Response, Status};

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
        let mut operate = ModuleOperate::from(request.into_inner());
        operate.operate_type = OperateType::Register;
        self.operate_module_sender.send(operate).unwrap();
        Ok(Response::new(Resp {
            code: RespCode::Success.into(),
        }))
    }

    async fn offline(&self, request: Request<SubmoduleReq>) -> Result<Response<Resp>, Status> {
        let mut operate = ModuleOperate::from(request.into_inner());
        operate.operate_type = OperateType::Offline;
        self.operate_module_sender.send(operate).unwrap();
        Ok(Response::new(Resp {
            code: RespCode::Success.into(),
        }))
    }

    async fn heartbeat(
        &self,
        request: Request<SubmoduleHeartbeat>,
    ) -> Result<Response<Resp>, Status> {
        self.operate_module_sender
            .send(ModuleOperate::from(request.into_inner()))
            .unwrap();
        Ok(Response::new(Resp {
            code: RespCode::Success.into(),
        }))
    }

    async fn update(&self, request: Request<SubmoduleReq>) -> Result<Response<Resp>, Status> {
        let mut operate = ModuleOperate::from(request.into_inner());
        operate.operate_type = OperateType::Update;
        self.operate_module_sender.send(operate).unwrap();
        Ok(Response::new(Resp {
            code: RespCode::Success.into(),
        }))
    }
}

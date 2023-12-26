use std::collections::HashMap;

use crate::error::NihilityCommonError;
use crate::submodule::{ReceiveType, SubmoduleHeartbeat, SubmoduleReq, SubmoduleType};

#[derive(Debug)]
pub enum ConnectionType {
    GrpcType,
    PipeType,
    WindowsNamedPipeType,
    HttpType,
}

#[derive(Debug)]
pub enum ClientType {
    BothType,
    InstructType,
    ManipulateType,
}

#[derive(Debug)]
pub enum OperateType {
    Undefined,
    Register,
    Offline,
    Heartbeat,
    Update,
}

#[derive(Debug)]
pub struct SubmoduleInfo {
    pub default_instruct: Vec<String>,
    pub connection_type: ConnectionType,
    pub client_type: ClientType,
    pub conn_params: HashMap<String, String>,
}

#[derive(Debug)]
pub struct ModuleOperate {
    pub name: String,
    pub info: Option<SubmoduleInfo>,
    pub operate_type: OperateType,
}

impl From<SubmoduleType> for ConnectionType {
    fn from(value: SubmoduleType) -> Self {
        match value {
            SubmoduleType::GrpcType => ConnectionType::GrpcType,
            SubmoduleType::PipeType => ConnectionType::PipeType,
            SubmoduleType::WindowsNamedPipeType => ConnectionType::WindowsNamedPipeType,
            SubmoduleType::HttpType => ConnectionType::HttpType,
        }
    }
}

impl From<ReceiveType> for ClientType {
    fn from(value: ReceiveType) -> Self {
        match value {
            ReceiveType::DefaultType => ClientType::BothType,
            ReceiveType::JustInstructType => ClientType::InstructType,
            ReceiveType::JustManipulateType => ClientType::ManipulateType,
        }
    }
}

impl From<SubmoduleReq> for ModuleOperate {
    fn from(value: SubmoduleReq) -> Self {
        let connection_type = ConnectionType::from(value.submodule_type());
        let client_type = ClientType::from(value.receive_type());
        ModuleOperate {
            name: value.name,
            info: Some(SubmoduleInfo {
                default_instruct: value.default_instruct,
                connection_type,
                client_type,
                conn_params: value.conn_params,
            }),
            operate_type: OperateType::Undefined,
        }
    }
}

impl From<SubmoduleHeartbeat> for ModuleOperate {
    fn from(value: SubmoduleHeartbeat) -> Self {
        ModuleOperate {
            name: value.name,
            info: None,
            operate_type: OperateType::Heartbeat,
        }
    }
}

impl From<ConnectionType> for SubmoduleType {
    fn from(value: ConnectionType) -> Self {
        match value {
            ConnectionType::GrpcType => SubmoduleType::GrpcType,
            ConnectionType::PipeType => SubmoduleType::PipeType,
            ConnectionType::WindowsNamedPipeType => SubmoduleType::WindowsNamedPipeType,
            ConnectionType::HttpType => SubmoduleType::HttpType,
        }
    }
}

impl From<ClientType> for ReceiveType {
    fn from(value: ClientType) -> Self {
        match value {
            ClientType::BothType => ReceiveType::DefaultType,
            ClientType::InstructType => ReceiveType::JustInstructType,
            ClientType::ManipulateType => ReceiveType::JustManipulateType,
        }
    }
}

impl TryInto<SubmoduleReq> for ModuleOperate {
    type Error = NihilityCommonError;

    fn try_into(self) -> Result<SubmoduleReq, Self::Error> {
        if let Some(info) = self.info {
            Ok(SubmoduleReq {
                name: self.name,
                receive_type: ReceiveType::from(info.client_type).into(),
                submodule_type: SubmoduleType::from(info.connection_type).into(),
                conn_params: info.conn_params,
                default_instruct: info.default_instruct,
            })
        } else {
            Err(NihilityCommonError::CreateSubmoduleReq)
        }
    }
}

impl TryInto<SubmoduleHeartbeat> for ModuleOperate {
    type Error = NihilityCommonError;

    fn try_into(self) -> Result<SubmoduleHeartbeat, Self::Error> {
        match self.operate_type {
            OperateType::Heartbeat => Ok(SubmoduleHeartbeat { name: self.name }),
            other_type => Err(NihilityCommonError::CreateSubmoduleHeartbeat(other_type)),
        }
    }
}

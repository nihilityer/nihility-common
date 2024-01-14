use std::collections::HashMap;

use serde::Serialize;

use nihility_procmacro::Sign;

use crate::error::NihilityCommonError;
use crate::submodule::{
    ConnectionParams, ReceiveType, SubmoduleHeartbeat, SubmoduleReq, SubmoduleType,
};
use crate::utils::auth::{get_auth_id_bytes, Signature};

#[derive(Debug, Default, Serialize)]
pub enum ConnectionType {
    #[default]
    GrpcType,
    PipeType,
    WindowsNamedPipeType,
    HttpType,
}

#[derive(Debug, Default, Serialize)]
pub enum ClientType {
    #[default]
    NotReceiveType,
    BothType,
    InstructType,
    ManipulateType,
}

#[derive(Debug, Default, Serialize)]
pub enum OperateType {
    #[default]
    Undefined,
    Register,
    Offline,
    Heartbeat,
    Update,
}

#[derive(Debug, Serialize)]
pub struct ConnParams {
    pub connection_type: ConnectionType,
    pub client_type: ClientType,
    pub conn_config: HashMap<String, String>,
}

#[derive(Debug, Serialize)]
pub struct SubmoduleInfo {
    pub default_instruct: Vec<String>,
    pub conn_params: ConnParams,
}

#[derive(Debug, Serialize, Sign)]
pub struct ModuleOperate {
    pub name: String,
    pub info: Option<SubmoduleInfo>,
    pub operate_type: OperateType,
    sign: Vec<u8>,
}

impl Default for ModuleOperate {
    fn default() -> Self {
        ModuleOperate {
            name: String::default(),
            info: None,
            operate_type: OperateType::default(),
            sign: get_auth_id_bytes(),
        }
    }
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
            ReceiveType::NotReceiveType => ClientType::NotReceiveType,
        }
    }
}

impl From<ConnectionParams> for ConnParams {
    fn from(value: ConnectionParams) -> Self {
        let connection_type = ConnectionType::from(value.submodule_type());
        let client_type = ClientType::from(value.receive_type());
        ConnParams {
            connection_type,
            client_type,
            conn_config: value.conn_params,
        }
    }
}

impl From<ConnParams> for ConnectionParams {
    fn from(value: ConnParams) -> Self {
        ConnectionParams {
            submodule_type: SubmoduleType::from(value.connection_type).into(),
            receive_type: ReceiveType::from(value.client_type).into(),
            conn_params: value.conn_config,
        }
    }
}

impl TryFrom<SubmoduleReq> for ModuleOperate {
    type Error = NihilityCommonError;

    fn try_from(value: SubmoduleReq) -> Result<Self, Self::Error> {
        match value.connection_params {
            None => Err(NihilityCommonError::CreateSubmoduleReq),
            Some(connection_params) => Ok(ModuleOperate {
                name: value.name,
                info: Some(SubmoduleInfo {
                    default_instruct: value.default_instruct,
                    conn_params: ConnParams::from(connection_params),
                }),
                operate_type: OperateType::Undefined,
                sign: value.sign,
            }),
        }
    }
}

impl From<SubmoduleHeartbeat> for ModuleOperate {
    fn from(value: SubmoduleHeartbeat) -> Self {
        ModuleOperate {
            name: value.name,
            info: None,
            operate_type: OperateType::Heartbeat,
            sign: value.sign,
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
            ClientType::NotReceiveType => ReceiveType::NotReceiveType,
        }
    }
}

impl TryInto<SubmoduleReq> for ModuleOperate {
    type Error = NihilityCommonError;

    fn try_into(self) -> Result<SubmoduleReq, Self::Error> {
        if let Some(info) = self.info {
            Ok(SubmoduleReq {
                name: self.name,
                connection_params: Some(ConnectionParams::from(info.conn_params)),
                default_instruct: info.default_instruct,
                sign: self.sign,
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
            OperateType::Heartbeat => Ok(SubmoduleHeartbeat {
                name: self.name,
                sign: self.sign,
            }),
            other_type => Err(NihilityCommonError::CreateSubmoduleHeartbeat(other_type)),
        }
    }
}

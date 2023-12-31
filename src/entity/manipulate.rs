use uuid::Uuid;

use crate::entity::submodule::ConnParams;
use crate::error::NihilityCommonError;
use crate::error::NihilityCommonError::CreateManipulateReq;
use crate::manipulate::{
    DirectConnectionManipulate, ManipulateInfo, SimpleManipulate, TextDisplayManipulate, Type,
};
use crate::submodule::ConnectionParams;

#[derive(Debug, Clone)]
pub enum ManipulateType {
    DefaultType,
    OfflineType,
    ConfirmType,
    CancelType,
    ConnectionType,
    DisconnectionType,
}

#[derive(Debug, Clone)]
pub struct ManipulateInfoEntity {
    pub manipulate_id: String,
    pub manipulate_type: ManipulateType,
    pub use_module_name: String,
}

#[derive(Debug)]
pub enum ManipulateData {
    Text(String),
    Simple,
    ConnectionParams(Box<ConnParams>),
}

/// 核心模块内部传递的操作实体
#[derive(Debug)]
pub struct ManipulateEntity {
    pub info: ManipulateInfoEntity,
    pub manipulate: ManipulateData,
}

impl From<Type> for ManipulateType {
    fn from(value: Type) -> Self {
        match value {
            Type::DefaultType => ManipulateType::DefaultType,
            Type::OfflineType => ManipulateType::OfflineType,
            Type::ConfirmType => ManipulateType::ConfirmType,
            Type::CancelType => ManipulateType::CancelType,
            Type::ConnectionType => ManipulateType::ConnectionType,
            Type::DisconnectionType => ManipulateType::DisconnectionType,
        }
    }
}

impl From<ManipulateType> for Type {
    fn from(value: ManipulateType) -> Self {
        match value {
            ManipulateType::DefaultType => Type::DefaultType,
            ManipulateType::OfflineType => Type::OfflineType,
            ManipulateType::ConfirmType => Type::ConfirmType,
            ManipulateType::CancelType => Type::CancelType,
            ManipulateType::ConnectionType => Type::ConnectionType,
            ManipulateType::DisconnectionType => Type::DisconnectionType,
        }
    }
}

impl Default for ManipulateInfoEntity {
    fn default() -> Self {
        ManipulateInfoEntity {
            manipulate_id: Uuid::new_v4().to_string(),
            manipulate_type: ManipulateType::DefaultType,
            use_module_name: String::default(),
        }
    }
}

impl From<ManipulateInfo> for ManipulateInfoEntity {
    fn from(value: ManipulateInfo) -> Self {
        ManipulateInfoEntity {
            manipulate_type: ManipulateType::from(value.manipulate_type()),
            manipulate_id: value.manipulate_id,
            use_module_name: value.use_module_name,
        }
    }
}

impl From<ManipulateInfoEntity> for ManipulateInfo {
    fn from(value: ManipulateInfoEntity) -> Self {
        ManipulateInfo {
            manipulate_id: value.manipulate_id,
            manipulate_type: Type::from(value.manipulate_type).into(),
            use_module_name: value.use_module_name,
        }
    }
}

impl From<TextDisplayManipulate> for ManipulateEntity {
    fn from(value: TextDisplayManipulate) -> Self {
        match value.info {
            None => ManipulateEntity {
                info: ManipulateInfoEntity::default(),
                manipulate: ManipulateData::Text(value.text),
            },
            Some(info) => ManipulateEntity {
                info: info.into(),
                manipulate: ManipulateData::Text(value.text),
            },
        }
    }
}

impl TryInto<TextDisplayManipulate> for ManipulateEntity {
    type Error = NihilityCommonError;

    fn try_into(self) -> Result<TextDisplayManipulate, Self::Error> {
        match self.manipulate {
            ManipulateData::Text(text) => Ok(TextDisplayManipulate {
                info: Some(self.info.into()),
                text,
            }),
            other_type => Err(CreateManipulateReq(other_type)),
        }
    }
}

impl From<SimpleManipulate> for ManipulateEntity {
    fn from(value: SimpleManipulate) -> Self {
        match value.info {
            None => ManipulateEntity {
                info: ManipulateInfoEntity::default(),
                manipulate: ManipulateData::Simple,
            },
            Some(info) => ManipulateEntity {
                info: info.into(),
                manipulate: ManipulateData::Simple,
            },
        }
    }
}

impl TryInto<SimpleManipulate> for ManipulateEntity {
    type Error = NihilityCommonError;

    fn try_into(self) -> Result<SimpleManipulate, Self::Error> {
        match self.manipulate {
            ManipulateData::Simple => Ok(SimpleManipulate {
                info: Some(self.info.into()),
            }),
            other_type => Err(CreateManipulateReq(other_type)),
        }
    }
}

impl TryFrom<DirectConnectionManipulate> for ManipulateEntity {
    type Error = NihilityCommonError;

    fn try_from(value: DirectConnectionManipulate) -> Result<Self, Self::Error> {
        match value.connection_params {
            None => Err(NihilityCommonError::CreateManipulateEntity),
            Some(connection_params) => Ok(match value.info {
                None => ManipulateEntity {
                    info: ManipulateInfoEntity::default(),
                    manipulate: ManipulateData::ConnectionParams(Box::new(ConnParams::from(
                        connection_params,
                    ))),
                },
                Some(info) => ManipulateEntity {
                    info: info.into(),
                    manipulate: ManipulateData::Simple,
                },
            }),
        }
    }
}

impl TryInto<DirectConnectionManipulate> for ManipulateEntity {
    type Error = NihilityCommonError;

    fn try_into(self) -> Result<DirectConnectionManipulate, Self::Error> {
        match self.manipulate {
            ManipulateData::ConnectionParams(connection_params) => Ok(DirectConnectionManipulate {
                info: Some(self.info.into()),
                connection_params: Some(ConnectionParams::from(*connection_params)),
            }),
            other_type => Err(CreateManipulateReq(other_type)),
        }
    }
}

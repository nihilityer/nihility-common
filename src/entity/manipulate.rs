use crate::DEFAULT_INSTRUCT_HANDLER_SUBMODULE_NAME;
use crate::error::NihilityCommonError;
use crate::error::NihilityCommonError::CreateManipulateReq;
use crate::manipulate::{ManipulateInfo, ManipulateType, SimpleManipulate, TextDisplayManipulate};

#[derive(Debug, Clone)]
pub enum Type {
    DefaultType,
    SpecialType,
    OfflineType,
    DiscontinueType,
    CancelType,
    ConfirmType,
    WaitNextType,
}

#[derive(Debug, Clone)]
pub struct ManipulateInfoEntity {
    pub manipulate_type: Type,
    pub use_module_name: String,
}

#[derive(Debug)]
pub enum ManipulateData {
    Text(String),
    Simple,
}

/// 核心模块内部传递的操作实体
#[derive(Debug)]
pub struct ManipulateEntity {
    pub info: ManipulateInfoEntity,
    pub manipulate: ManipulateData,
}

impl From<ManipulateType> for Type {
    fn from(value: ManipulateType) -> Self {
        match value {
            ManipulateType::DefaultType => Type::DefaultType,
            ManipulateType::SpecialType => Type::SpecialType,
            ManipulateType::OfflineType => Type::OfflineType,
            ManipulateType::DiscontinueType => Type::DiscontinueType,
            ManipulateType::CancelType => Type::CancelType,
            ManipulateType::ConfirmType => Type::ConfirmType,
            ManipulateType::WaitNextType => Type::WaitNextType,
        }
    }
}

impl Into<ManipulateType> for Type {
    fn into(self) -> ManipulateType {
        match self {
            Type::DefaultType => ManipulateType::DefaultType,
            Type::SpecialType => ManipulateType::SpecialType,
            Type::OfflineType => ManipulateType::OfflineType,
            Type::DiscontinueType => ManipulateType::DiscontinueType,
            Type::CancelType => ManipulateType::CancelType,
            Type::ConfirmType => ManipulateType::ConfirmType,
            Type::WaitNextType => ManipulateType::WaitNextType,
        }
    }
}

impl Default for ManipulateInfoEntity {
    fn default() -> Self {
        ManipulateInfoEntity {
            manipulate_type: Type::DefaultType,
            use_module_name: DEFAULT_INSTRUCT_HANDLER_SUBMODULE_NAME.get().unwrap().to_string(),
        }
    }
}

impl From<ManipulateInfo> for ManipulateInfoEntity {
    fn from(value: ManipulateInfo) -> Self {
        ManipulateInfoEntity {
            manipulate_type: Type::from(value.manipulate_type()),
            use_module_name: value.use_module_name,
        }
    }
}

impl Into<ManipulateInfo> for ManipulateInfoEntity {
    fn into(self) -> ManipulateInfo {
        ManipulateInfo {
            manipulate_type: <Type as Into<ManipulateType>>::into(self.manipulate_type).into(),
            use_module_name: self.use_module_name,
        }
    }
}

impl From<TextDisplayManipulate> for ManipulateEntity {
    fn from(value: TextDisplayManipulate) -> Self {
        match value.info {
            None => {
                ManipulateEntity {
                    info: ManipulateInfoEntity::default(),
                    manipulate: ManipulateData::Text(value.text),
                }
            }
            Some(info) => {
                ManipulateEntity {
                    info: info.into(),
                    manipulate: ManipulateData::Text(value.text),
                }
            }
        }
    }
}

impl TryInto<TextDisplayManipulate> for ManipulateEntity {
    type Error = NihilityCommonError;

    fn try_into(self) -> Result<TextDisplayManipulate, Self::Error> {
        match self.manipulate {
            ManipulateData::Text(text) => {
                Ok(TextDisplayManipulate {
                    info: Some(self.info.into()),
                    text,
                })
            }
            other_type => Err(CreateManipulateReq(other_type))
        }
    }
}

impl From<SimpleManipulate> for ManipulateEntity {
    fn from(value: SimpleManipulate) -> Self {
        match value.info {
            None => {
                ManipulateEntity {
                    info: ManipulateInfoEntity::default(),
                    manipulate: ManipulateData::Simple,
                }
            }
            Some(info) => {
                ManipulateEntity {
                    info: info.into(),
                    manipulate: ManipulateData::Simple,
                }
            }
        }
    }
}

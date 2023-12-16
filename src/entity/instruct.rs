use crate::DEFAULT_RECEIVER_SUBMODULE_NAME;
use crate::error::NihilityCommonError;
use crate::instruct::{InstructInfo, InstructType, TextInstruct};

#[derive(Debug)]
pub enum Type {
    DefaultType,
    SpecialType,
    WaitNextType,
}

#[derive(Debug)]
pub struct InstructInfoEntity {
    pub instruct_type: Type,
    pub receive_manipulate_submodule: String,
}

#[derive(Debug, Clone)]
pub enum InstructData {
    Text(String),
}

/// 核心心模块内部传递的指令实体
#[derive(Debug)]
pub struct InstructEntity {
    pub info: InstructInfoEntity,
    pub instruct: InstructData,
}

impl From<InstructType> for Type {
    fn from(value: InstructType) -> Self {
        match value {
            InstructType::DefaultType => Type::DefaultType,
            InstructType::SpecialType => Type::SpecialType,
            InstructType::WaitNextType => Type::WaitNextType,
        }
    }
}

impl Into<InstructType> for Type {
    fn into(self) -> InstructType {
        match self {
            Type::DefaultType => InstructType::DefaultType,
            Type::SpecialType => InstructType::SpecialType,
            Type::WaitNextType => InstructType::WaitNextType,
        }
    }
}

impl Default for InstructInfoEntity {
    fn default() -> Self {
        InstructInfoEntity {
            instruct_type: Type::DefaultType,
            receive_manipulate_submodule: DEFAULT_RECEIVER_SUBMODULE_NAME.get().unwrap().to_string(),
        }
    }
}

impl From<InstructInfo> for InstructInfoEntity {
    fn from(value: InstructInfo) -> Self {
        InstructInfoEntity {
            instruct_type: Type::from(value.instruct_type()),
            receive_manipulate_submodule: value.receive_manipulate_submodule,
        }
    }
}

impl Into<InstructInfo> for InstructInfoEntity {
    fn into(self) -> InstructInfo {
        InstructInfo {
            instruct_type: <Type as Into<InstructType>>::into(self.instruct_type).into(),
            receive_manipulate_submodule: self.receive_manipulate_submodule,
        }
    }
}

impl From<TextInstruct> for InstructEntity {
    fn from(value: TextInstruct) -> Self {
        match value.info {
            None => {
                InstructEntity {
                    info: InstructInfoEntity::default(),
                    instruct: InstructData::Text(value.instruct),
                }
            }
            Some(info) => {
                InstructEntity {
                    info: info.into(),
                    instruct: InstructData::Text(value.instruct),
                }
            }
        }
    }
}

impl TryInto<TextInstruct> for InstructEntity {
    type Error = NihilityCommonError;

    fn try_into(self) -> Result<TextInstruct, Self::Error> {
        match self.instruct {
            InstructData::Text(text) => {
                Ok(TextInstruct {
                    info: Some(self.info.into()),
                    instruct: text,
                })
            }
        }
    }
}

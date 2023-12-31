use crate::error::NihilityCommonError;
use crate::instruct::{InstructInfo, TextInstruct, Type};
use uuid::Uuid;

#[derive(Debug)]
pub enum InstructType {
    DefaultType,
    SpecialType,
    WaitNextType,
}

#[derive(Debug)]
pub struct InstructInfoEntity {
    pub instruct_id: String,
    pub instruct_type: InstructType,
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

impl From<Type> for InstructType {
    fn from(value: Type) -> Self {
        match value {
            Type::DefaultType => InstructType::DefaultType,
            Type::SpecialType => InstructType::SpecialType,
            Type::WaitNextType => InstructType::WaitNextType,
        }
    }
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

impl Default for InstructInfoEntity {
    fn default() -> Self {
        InstructInfoEntity {
            instruct_id: Uuid::new_v4().to_string(),
            instruct_type: InstructType::DefaultType,
            receive_manipulate_submodule: String::default(),
        }
    }
}

impl From<InstructInfo> for InstructInfoEntity {
    fn from(value: InstructInfo) -> Self {
        InstructInfoEntity {
            instruct_type: InstructType::from(value.instruct_type()),
            instruct_id: value.instruct_id,
            receive_manipulate_submodule: value.receive_manipulate_submodule,
        }
    }
}

impl From<InstructInfoEntity> for InstructInfo {
    fn from(value: InstructInfoEntity) -> Self {
        InstructInfo {
            instruct_id: value.instruct_id,
            instruct_type: Type::from(value.instruct_type).into(),
            receive_manipulate_submodule: value.receive_manipulate_submodule,
        }
    }
}

impl From<TextInstruct> for InstructEntity {
    fn from(value: TextInstruct) -> Self {
        match value.info {
            None => InstructEntity {
                info: InstructInfoEntity::default(),
                instruct: InstructData::Text(value.instruct),
            },
            Some(info) => InstructEntity {
                info: info.into(),
                instruct: InstructData::Text(value.instruct),
            },
        }
    }
}

impl TryInto<TextInstruct> for InstructEntity {
    type Error = NihilityCommonError;

    fn try_into(self) -> Result<TextInstruct, Self::Error> {
        match self.instruct {
            InstructData::Text(text) => Ok(TextInstruct {
                info: Some(self.info.into()),
                instruct: text,
            }),
        }
    }
}

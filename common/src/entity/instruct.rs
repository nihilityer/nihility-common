use std::fmt;
use std::fmt::Formatter;

use serde::Serialize;
use uuid::Uuid;

use nihility_procmacro::Sign;

use crate::error::NihilityCommonError;
use crate::get_default_receiver_submodule;
use crate::instruct::{InstructInfo, TextInstruct, Type};
use crate::utils::auth::{get_auth_id_bytes, Signature};

#[derive(Debug, Default, Serialize)]
pub enum InstructType {
    #[default]
    DefaultType,
    SpecialType,
    WaitNextType,
}

#[derive(Debug, Serialize)]
pub struct InstructInfoEntity {
    pub instruct_id: String,
    pub instruct_type: InstructType,
    pub receive_manipulate_submodule: String,
}

#[derive(Debug, Clone, Serialize)]
pub enum InstructData {
    Text(String),
}

#[derive(Serialize, Sign)]
pub struct InstructEntity {
    pub info: InstructInfoEntity,
    pub instruct: InstructData,
    sign: Vec<u8>,
}

impl InstructEntity {
    pub fn new_text(text: String) -> Self {
        InstructEntity {
            info: InstructInfoEntity::default(),
            instruct: InstructData::Text(text),
            sign: get_auth_id_bytes(),
        }
    }
}

impl fmt::Debug for InstructEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Instruct ( info: {:?}, instruct: {:?} )",
            self.info, self.instruct,
        )
    }
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
            receive_manipulate_submodule: get_default_receiver_submodule(),
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
                sign: value.sign,
            },
            Some(info) => InstructEntity {
                info: info.into(),
                instruct: InstructData::Text(value.instruct),
                sign: value.sign,
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
                sign: self.sign,
            }),
        }
    }
}

impl Default for InstructData {
    fn default() -> Self {
        InstructData::Text(String::new())
    }
}

use std::collections::HashMap;

use crate::submodule::{ReceiveType, SubmoduleReq, SubmoduleType};

/// 操作子模块类型
#[derive(Debug)]
pub enum OperateType {
    /// 注册当前模块
    Register,
    /// 注销当前模块
    Offline,
    /// 当前模块心跳信息
    Heartbeat,
    /// 更新当前模块
    Update,
}

/// 操作子模块消息结构体
#[derive(Debug)]
pub struct ModuleOperate {
    pub name: String,
    pub default_instruct: Vec<String>,
    pub submodule_type: SubmoduleType,
    pub receive_type: ReceiveType,
    pub conn_params: HashMap<String, String>,
    pub operate_type: OperateType,
}

impl ModuleOperate {
    /// 通过应用间消息创建操作子模块消息结构体，由调用的方法决定结构体类型
    pub fn create_by_req(req: SubmoduleReq, operate_type: OperateType) -> Self {
        ModuleOperate {
            name: req.name.clone(),
            default_instruct: req.default_instruct.clone(),
            submodule_type: req.clone().submodule_type(),
            receive_type: req.clone().receive_type(),
            conn_params: req.conn_params.clone(),
            operate_type,
        }
    }
}

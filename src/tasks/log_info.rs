use std::fmt::Debug;

pub const MSG_TASK_EXECUTED_OK: &'static str = "task executed ok";
#[allow(dead_code)]
pub const MSG_TRANSFER_TASK_START: &'static str = "Transfer task start";

/// 日志信息结构体，用于记录任务的执行日志
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct LogInfo<T> {
    /// 任务唯一标识符
    pub task_id: String,
    /// 日志消息内容
    pub msg: String,
    /// 附加信息（可选）
    pub additional: Option<T>,
}

pub fn log_info<T>(id: String, msg: String, additional: Option<T>)
where
    T: Debug,
{
    let info = LogInfo {
        task_id: id,
        msg,
        additional,
    };
    log::info!("{:#?}", info);
}

use std::fmt::Debug;

pub const MSG_TASK_EXECUTED_OK: &'static str = "task executed ok";
pub const MSG_TRANSFER_TASK_START: &'static str = "Transfer task start";

#[derive(Debug, Clone)]
pub struct LogInfo<T> {
    pub task_id: String,
    pub msg: String,
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

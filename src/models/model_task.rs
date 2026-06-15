use crate::models::{
    model_s3::OSSDescription, model_task_compare::CompareTask,
    model_task_delete_bucket::TaskDeleteBucket, model_task_transfer::TransferTask,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
/// 表示分析结果的结构体，包含最大值和最小值
pub struct AnalyzedResult {
    /// 分析结果中的最大值
    pub max: i128,
    /// 分析结果中的最小值
    pub min: i128,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
#[serde(rename_all = "lowercase")]
// #[serde(tag = "type")]
pub enum ObjectStorage {
    Local(String),
    OSS(OSSDescription),
}

impl Default for ObjectStorage {
    fn default() -> Self {
        ObjectStorage::OSS(OSSDescription::default())
    }
}

/// 任务阶段，包括存量曾量全量
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum TaskStage {
    Stock,
    Increment,
}

/// 任务类别，根据传输方式划分
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum TaskType {
    Transfer,
    DeleteBucket,
    Compare,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
// #[serde(untagged)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type")]
pub enum Task {
    Transfer(TransferTask),
    Compare(CompareTask),
    DeleteBucket(TaskDeleteBucket),
}

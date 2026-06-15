use crate::models::{
    model_filters::LastModifyFilter, model_s3::OSSDescription, model_task::ObjectStorage,
    model_task_default_parameters::TaskDefaultParameters,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
/// 删除任务的属性
pub struct DeleteTaskAttributes {
    /// 每个批次处理的对象数量，默认为 TaskDefaultParameters::objects_per_batch_default()
    #[serde(default = "TaskDefaultParameters::objects_transfer_batch_default")]
    pub objects_per_batch: i32,
    /// 任务并行度，默认为 TaskDefaultParameters::task_parallelism_default()
    #[serde(default = "TaskDefaultParameters::task_parallelism_default")]
    pub task_parallelism: usize,
    /// 元数据目录，默认为 TaskDefaultParameters::meta_dir_default()
    #[serde(default = "TaskDefaultParameters::meta_dir_default")]
    pub meta_dir: String,
    /// 是否从检查点开始，默认为 TaskDefaultParameters::start_from_checkpoint_default()
    #[serde(default = "TaskDefaultParameters::start_from_checkpoint_default")]
    pub start_from_checkpoint: bool,
    /// 排除的对象列表，默认为 TaskDefaultParameters::filter_default()
    #[serde(default = "TaskDefaultParameters::filter_default")]
    pub exclude: Option<Vec<String>>,
    /// 包含的对象列表，默认为 TaskDefaultParameters::filter_default()
    #[serde(default = "TaskDefaultParameters::filter_default")]
    pub include: Option<Vec<String>>,
    /// 最后修改时间过滤器，默认为 TaskDefaultParameters::last_modify_filter_default()
    #[serde(default = "TaskDefaultParameters::last_modify_filter_default")]
    pub last_modify_filter: Option<LastModifyFilter>,
    /// 对象列表文件最大行数
    #[serde(default = "TaskDefaultParameters::objects_list_files_max_line_default")]
    pub objects_list_files_max_line: usize,
}

impl Default for DeleteTaskAttributes {
    fn default() -> Self {
        Self {
            objects_per_batch: TaskDefaultParameters::objects_transfer_batch_default(),
            task_parallelism: TaskDefaultParameters::task_parallelism_default(),
            exclude: TaskDefaultParameters::filter_default(),
            include: TaskDefaultParameters::filter_default(),
            last_modify_filter: TaskDefaultParameters::last_modify_filter_default(),
            meta_dir: TaskDefaultParameters::meta_dir_default(),
            start_from_checkpoint: TaskDefaultParameters::start_from_checkpoint_default(),
            objects_list_files_max_line: TaskDefaultParameters::objects_list_files_max_line_default(
            ),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskDeleteBucket {
    #[serde(default = "TaskDefaultParameters::id_default")]
    pub task_id: String,
    #[serde(default = "TaskDefaultParameters::name_default")]
    pub name: String,
    pub target: ObjectStorage,
    pub attributes: DeleteTaskAttributes,
}

impl Default for TaskDeleteBucket {
    fn default() -> Self {
        Self {
            task_id: TaskDefaultParameters::id_default(),
            name: TaskDefaultParameters::name_default(),
            target: ObjectStorage::OSS(OSSDescription::default()),
            attributes: DeleteTaskAttributes::default(),
        }
    }
}

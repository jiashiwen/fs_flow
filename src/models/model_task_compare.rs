use crate::models::model_filters::LastModifyFilter;
use crate::models::{
    model_task::ObjectStorage, model_task_default_parameters::TaskDefaultParameters,
};
use crate::tasks::{de_usize_from_str, se_usize_to_str};
use serde::{Deserialize, Serialize};

/// 比较任务属性，配置比较任务的各项参数
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CompareTaskAttributes {
    /// 每批次处理的对象数量
    #[serde(default = "TaskDefaultParameters::objects_compare_batch_default")]
    pub objects_per_batch: i32,
    /// 任务并行度
    #[serde(default = "TaskDefaultParameters::task_parallelism_default")]
    pub task_parallelism: usize,
    /// 元数据存储目录路径
    #[serde(default = "TaskDefaultParameters::meta_dir_default")]
    pub meta_dir: String,
    /// 是否从检查点开始执行
    #[serde(default = "TaskDefaultParameters::target_exists_skip_default")]
    pub start_from_checkpoint: bool,
    /// 大文件大小阈值（字节）
    #[serde(default = "TaskDefaultParameters::large_file_size_default")]
    #[serde(serialize_with = "se_usize_to_str")]
    #[serde(deserialize_with = "de_usize_from_str")]
    pub large_file_size: usize,
    /// 分片比较的块大小（字节）
    #[serde(default = "TaskDefaultParameters::multi_part_chunk_size_default")]
    #[serde(serialize_with = "se_usize_to_str")]
    #[serde(deserialize_with = "de_usize_from_str")]
    pub multi_part_chunk: usize,
    /// 分片比较的最大并行度
    #[serde(default = "TaskDefaultParameters::multi_part_max_parallelism_default")]
    pub multi_part_max_parallelism: usize,
    /// 排除的文件/对象匹配模式列表
    #[serde(default = "TaskDefaultParameters::filter_default")]
    pub exclude: Option<Vec<String>>,
    /// 包含的文件/对象匹配模式列表
    #[serde(default = "TaskDefaultParameters::filter_default")]
    pub include: Option<Vec<String>>,
    /// 过期时间差异范围（秒）
    #[serde(default = "TaskDefaultParameters::exprirs_diff_scope_default")]
    pub exprirs_diff_scope: i64,
    /// 最后修改时间过滤器配置
    #[serde(default = "TaskDefaultParameters::last_modify_filter_default")]
    pub last_modify_filter: Option<LastModifyFilter>,
    /// 对象列表批次大小
    #[serde(default = "TaskDefaultParameters::objects_list_batch_default")]
    pub objects_list_batch: i32,
    /// 对象列表文件最大行数
    #[serde(default = "TaskDefaultParameters::objects_list_files_max_line_default")]
    pub objects_list_files_max_line: usize,
}

impl Default for CompareTaskAttributes {
    fn default() -> Self {
        Self {
            objects_per_batch: TaskDefaultParameters::objects_transfer_batch_default(),
            task_parallelism: TaskDefaultParameters::task_parallelism_default(),
            meta_dir: TaskDefaultParameters::meta_dir_default(),
            start_from_checkpoint: TaskDefaultParameters::target_exists_skip_default(),
            large_file_size: TaskDefaultParameters::large_file_size_default(),
            multi_part_chunk: TaskDefaultParameters::multi_part_chunk_size_default(),
            multi_part_max_parallelism: TaskDefaultParameters::multi_part_max_parallelism_default(),
            exclude: TaskDefaultParameters::filter_default(),
            include: TaskDefaultParameters::filter_default(),
            last_modify_filter: TaskDefaultParameters::last_modify_filter_default(),
            exprirs_diff_scope: TaskDefaultParameters::exprirs_diff_scope_default(),
            objects_list_files_max_line: TaskDefaultParameters::objects_list_files_max_line_default(
            ),
            objects_list_batch: TaskDefaultParameters::objects_list_batch_default(),
        }
    }
}

/// 比较检查选项，配置比较任务中各项检查的开关
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CompareCheckOption {
    /// 是否检查内容长度
    #[serde(default = "CompareCheckOption::default_check_content_length")]
    check_content_length: bool,
    /// 是否检查过期时间
    #[serde(default = "CompareCheckOption::default_check_expires")]
    check_expires: bool,
    /// 是否检查内容
    #[serde(default = "CompareCheckOption::default_check_content")]
    check_content: bool,
    /// 是否检查元数据
    #[serde(default = "CompareCheckOption::default_check_meta_data")]
    check_meta_data: bool,
}

impl Default for CompareCheckOption {
    fn default() -> Self {
        Self {
            check_content_length: CompareCheckOption::default_check_content_length(),
            check_expires: CompareCheckOption::default_check_expires(),
            check_content: CompareCheckOption::default_check_content(),
            check_meta_data: CompareCheckOption::default_check_meta_data(),
        }
    }
}

impl CompareCheckOption {
    pub fn default_check_content_length() -> bool {
        true
    }

    pub fn default_check_expires() -> bool {
        false
    }

    pub fn default_check_content() -> bool {
        false
    }

    pub fn default_check_meta_data() -> bool {
        false
    }

    pub fn check_content_length(&self) -> bool {
        self.check_content_length
    }

    pub fn check_expires(&self) -> bool {
        self.check_expires
    }

    pub fn check_meta_data(&self) -> bool {
        self.check_meta_data
    }

    pub fn check_content(&self) -> bool {
        self.check_content
    }
}

/// 比较任务，描述从源到目标的完整比较任务
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub struct CompareTask {
    /// 任务唯一标识符
    #[serde(default = "TaskDefaultParameters::id_default")]
    pub task_id: String,
    /// 任务名称
    #[serde(default = "TaskDefaultParameters::name_default")]
    pub name: String,
    /// 源存储端
    pub source: ObjectStorage,
    /// 目标存储端
    pub target: ObjectStorage,
    /// 比较检查选项
    pub check_option: CompareCheckOption,
    /// 比较任务属性
    pub attributes: CompareTaskAttributes,
}

impl Default for CompareTask {
    fn default() -> Self {
        Self {
            task_id: TaskDefaultParameters::id_default(),
            name: TaskDefaultParameters::name_default(),
            source: ObjectStorage::default(),
            target: ObjectStorage::default(),
            check_option: CompareCheckOption::default(),
            attributes: CompareTaskAttributes::default(),
        }
    }
}

use crate::models::model_filters::LastModifyFilter;
use crate::models::{
    model_task::ObjectStorage, model_task_default_parameters::TaskDefaultParameters,
};
use crate::tasks::{de_usize_from_str, se_usize_to_str};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CompareTaskAttributes {
    #[serde(default = "TaskDefaultParameters::objects_compare_batch_default")]
    pub objects_per_batch: i32,
    #[serde(default = "TaskDefaultParameters::task_parallelism_default")]
    pub task_parallelism: usize,
    #[serde(default = "TaskDefaultParameters::meta_dir_default")]
    pub meta_dir: String,
    #[serde(default = "TaskDefaultParameters::target_exists_skip_default")]
    pub start_from_checkpoint: bool,
    #[serde(default = "TaskDefaultParameters::large_file_size_default")]
    #[serde(serialize_with = "se_usize_to_str")]
    #[serde(deserialize_with = "de_usize_from_str")]
    pub large_file_size: usize,
    #[serde(default = "TaskDefaultParameters::multi_part_chunk_size_default")]
    #[serde(serialize_with = "se_usize_to_str")]
    #[serde(deserialize_with = "de_usize_from_str")]
    pub multi_part_chunk: usize,
    #[serde(default = "TaskDefaultParameters::multi_part_max_parallelism_default")]
    pub multi_part_max_parallelism: usize,
    #[serde(default = "TaskDefaultParameters::filter_default")]
    pub exclude: Option<Vec<String>>,
    #[serde(default = "TaskDefaultParameters::filter_default")]
    pub include: Option<Vec<String>>,
    #[serde(default = "TaskDefaultParameters::exprirs_diff_scope_default")]
    pub exprirs_diff_scope: i64,
    #[serde(default = "TaskDefaultParameters::last_modify_filter_default")]
    pub last_modify_filter: Option<LastModifyFilter>,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CompareCheckOption {
    #[serde(default = "CompareCheckOption::default_check_content_length")]
    check_content_length: bool,
    #[serde(default = "CompareCheckOption::default_check_expires")]
    check_expires: bool,
    #[serde(default = "CompareCheckOption::default_check_content")]
    check_content: bool,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub struct CompareTask {
    #[serde(default = "TaskDefaultParameters::id_default")]
    pub task_id: String,
    #[serde(default = "TaskDefaultParameters::name_default")]
    pub name: String,
    pub source: ObjectStorage,
    pub target: ObjectStorage,
    pub check_option: CompareCheckOption,
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

use crate::models::model_task::TaskStage;
use crate::models::model_task_default_parameters::TaskDefaultParameters;
use anyhow::{Context, Error, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

/// 文件描述信息，包含文件路径、大小和行数
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileDescription {
    /// 文件路径
    pub path: String,
    /// 文件大小（字节）
    pub size: u64,
    /// 文件总行数
    pub total_lines: u64,
}

impl Default for FileDescription {
    fn default() -> Self {
        Self {
            path: "".to_string(),
            size: 0,
            total_lines: 0,
        }
    }
}

impl FileDescription {
    pub fn from_file(file_path: &str) -> Result<Self> {
        let file = File::open(file_path)?;

        // 获取文件的字节数（通过元数据）
        let metadata = file.metadata()?;
        let byte_count = metadata.len();

        // 使用 BufReader 逐行读取文件内容
        let reader = BufReader::new(file);
        let line_count = reader.lines().count();
        Ok(FileDescription {
            path: file_path.to_string(),
            size: byte_count,
            total_lines: TryInto::<u64>::try_into(line_count)?,
        })
    }
}

/// 文件执行位置，记录在文件列表中的执行进度
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct FilePosition {
    /// 文件编号，可根据编号找到 sequence_file 中对应的文件名；
    /// 若为负数则在 checkpoint 中不变更当前文件，当前文件为增量文件，执行增量逻辑
    pub file_num: i32,
    /// 文件中的字节偏移位置
    pub offset: usize,
    /// 文件中的行号位置
    pub line_num: u64,
}

impl Default for FilePosition {
    fn default() -> Self {
        Self {
            offset: 0,
            line_num: 0,
            file_num: 0,
        }
    }
}

/// 检查点，用于记录任务的执行状态，支持断点续传
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CheckPoint {
    /// 任务唯一标识符
    pub task_id: String,
    /// 当前正在执行的对象列表文件信息
    /// 对象列表命名规则：OBJECT_LIST_FILE_PREFIX + 秒级 unix 时间戳 'objeclt_list_unixtimestampe'
    pub executing_file: FileDescription,
    /// 文件执行位置（偏移量和行号），用于断点续传
    pub executing_file_position: FilePosition,
    /// 用于增量同步的通知文件路径（可选）
    pub file_for_notify: Option<String>,
    /// 任务阶段（存量或增量）
    pub task_stage: TaskStage,
    /// 记录检查点的当前时间戳
    pub modify_checkpoint_timestamp: i128,
    /// 任务起始时间戳，用于后续增量任务
    pub task_begin_timestamp: i128,
    /// 上次扫描的时间戳
    pub last_scan_timestamp: i128,
}

impl Default for CheckPoint {
    fn default() -> Self {
        Self {
            task_id: TaskDefaultParameters::id_default(),
            executing_file: Default::default(),
            executing_file_position: FilePosition {
                offset: 0,
                line_num: 0,
                file_num: 0,
            },
            file_for_notify: Default::default(),
            task_stage: TaskStage::Stock,
            modify_checkpoint_timestamp: 0,
            task_begin_timestamp: 0,
            last_scan_timestamp: 0,
        }
    }
}

impl FromStr for CheckPoint {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let r = serde_yaml::from_str::<Self>(s).context(format!("{}:{}", file!(), line!()))?;
        Ok(r)
    }
}

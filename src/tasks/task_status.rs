// use super::TaskStage;
use crate::{
    checkpoint::get_task_checkpoint,
    consts::task_consts::OFFSET_PREFIX,
    models::{
        model_checkpoint::{CheckPoint, FileDescription, FilePosition},
        model_task::TaskStage,
    },
};
use dashmap::DashMap;
use std::{
    sync::{atomic::AtomicBool, Arc},
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::task::yield_now;

/// 表示任务状态保存器的结构体
pub struct TaskStatusSaver {
    /// 检查点文件的路径
    pub check_point_path: String,
    /// 已执行文件的描述信息
    pub executed_file: FileDescription,
    /// 用于标记任务是否已停止的原子布尔值
    pub stop_mark: Arc<AtomicBool>,
    /// 用于存储文件位置信息的哈希映射
    pub list_file_positon_map: Arc<DashMap<String, FilePosition>>,
    /// 用于通知的文件路径（可选）
    pub file_for_notify: Option<String>,
    /// 任务的当前阶段
    pub task_stage: TaskStage,
    /// 保存状态的时间间隔（以秒为单位）
    pub interval: u64,
    pub object_list_files: Option<Vec<FileDescription>>,
}

impl TaskStatusSaver {
    pub async fn snapshot_to_file(&self, task_id: String) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let mut checkpoint = match get_task_checkpoint(&self.check_point_path) {
            Ok(c) => c,
            Err(_) => {
                let mut checkpoint = CheckPoint {
                    task_id,
                    executing_file: self.executed_file.clone(),
                    executing_file_position: FilePosition {
                        file_num: 0,
                        offset: 0,
                        line_num: 0,
                    },
                    file_for_notify: self.file_for_notify.clone(),
                    task_stage: self.task_stage,
                    modify_checkpoint_timestamp: i128::from(now.as_secs()),
                    task_begin_timestamp: i128::from(now.as_secs()),
                    last_scan_timestamp: i128::from(now.as_secs()),
                };
                let _ = checkpoint.save_to(&self.check_point_path);
                checkpoint
            }
        };

        while !self.stop_mark.load(std::sync::atomic::Ordering::Relaxed) {
            let mut file_position = checkpoint.executing_file_position;
            let mut idx = 0;
            for item in self.list_file_positon_map.iter() {
                if item.key().starts_with(OFFSET_PREFIX) {
                    if idx.eq(&0) {
                        file_position = item.value().clone();
                        idx += 1;
                    } else {
                        if file_position.file_num > item.value().file_num {
                            file_position = item.value().clone();
                            continue;
                        }

                        if file_position.file_num.eq(&item.value().file_num) {
                            if file_position.offset > item.value().offset {
                                file_position = item.value().clone();
                            }
                        }
                    }
                }
            }

            self.list_file_positon_map.shrink_to_fit();
            checkpoint.executing_file_position = file_position.clone();

            if let Some(list_files) = &self.object_list_files {
                if let Ok(f_n) = TryInto::<usize>::try_into(file_position.file_num) {
                    if let Some(f_desc) = list_files.get(f_n) {
                        checkpoint.executing_file = f_desc.clone();
                    }
                }
            }

            if let Err(e) = checkpoint.save_to(&self.check_point_path) {
                log::error!("{:?},{:?}", e, self.check_point_path);
            } else {
                log::debug!("checkpoint:\n{:?}", checkpoint);
            };

            tokio::time::sleep(tokio::time::Duration::from_secs(self.interval)).await;
            yield_now().await;
        }
        log::info!("status saver stopped");
    }
}

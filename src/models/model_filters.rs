use regex::RegexSet;
use serde::{Deserialize, Serialize};

pub trait Filter<T> {
    fn intercepted(&self, content: T) -> bool;
}

#[derive(Debug, Clone)]
pub struct RegexFilter {
    pub exclude_regex: Option<RegexSet>,
    pub include_regex: Option<RegexSet>,
}

impl Default for RegexFilter {
    fn default() -> Self {
        Self {
            exclude_regex: None,
            include_regex: None,
        }
    }
}

/// 最后修改时间过滤器类型：Greater（大于指定时间戳）或 Less（小于指定时间戳）
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum LastModifyFilterType {
    /// 大于指定时间戳
    Greater,
    /// 小于指定时间戳
    Less,
}

/// 最后修改时间过滤器，根据文件的最后修改时间戳进行过滤
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct LastModifyFilter {
    /// 过滤类型（大于或小于）
    pub filter_type: LastModifyFilterType,
    /// 比较的时间戳
    pub timestamp: usize,
}

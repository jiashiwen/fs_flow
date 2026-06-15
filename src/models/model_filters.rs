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

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum LastModifyFilterType {
    Greater,
    Less,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct LastModifyFilter {
    pub filter_type: LastModifyFilterType,
    pub timestamp: usize,
}

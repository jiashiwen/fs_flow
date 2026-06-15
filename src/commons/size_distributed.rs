use std::collections::BTreeMap;

use dashmap::DashMap;

pub fn size_distributed(size: i128) -> String {
    return match size {
        s if s < 1024 => "0-1K".to_string(),
        s if s >= 1024 && s < 1024 * 8 => "1K-8K".to_string(),
        s if s >= 1024 * 8 && s < 1024 * 16 => "8K-16K".to_string(),
        s if s >= 1024 * 16 && s < 1024 * 32 => "16K-32K".to_string(),
        s if s >= 1024 * 32 && s < 1024 * 64 => "32K-64K".to_string(),
        s if s >= 1024 * 64 && s < 1024 * 128 => "64K-128K".to_string(),
        s if s >= 1024 * 128 && s < 1024 * 256 => "128K-256K".to_string(),
        s if s >= 1024 * 256 && s < 1024 * 512 => "256K-512K".to_string(),
        s if s >= 1024 * 512 && s < 1024 * 1024 => "512K-1M".to_string(),
        s if s >= 1024 * 1024 && s < 1024 * 1024 * 8 => "1M-8M".to_string(),
        s if s >= 1024 * 1024 * 8 && s < 1024 * 1024 * 16 => "8M-16M".to_string(),
        s if s >= 1024 * 1024 * 16 && s < 1024 * 1024 * 32 => "16M-32M".to_string(),
        s if s >= 1024 * 1024 * 32 && s < 1024 * 1024 * 64 => "32M-64M".to_string(),
        s if s >= 1024 * 1024 * 64 && s < 1024 * 1024 * 128 => "64M-128M".to_string(),
        s if s >= 1024 * 1024 * 128 && s < 1024 * 1024 * 256 => "128M-256M".to_string(),
        s if s >= 1024 * 1024 * 256 && s < 1024 * 1024 * 512 => "256M-512M".to_string(),
        s if s >= 1024 * 1024 * 512 && s < 1024 * 1024 * 1024 => "512M-1G".to_string(),
        s if s >= 1024 * 1024 * 1024 && s < 1024 * 1024 * 1024 * 8 => "1G-8G".to_string(),
        s if s >= 1024 * 1024 * 1024 * 8 && s < 1024 * 1024 * 1024 * 16 => "8G-16G".to_string(),
        s if s >= 1024 * 1024 * 1024 * 16 && s < 1024 * 1024 * 1024 * 32 => "16G-32G".to_string(),
        s if s >= 1024 * 1024 * 1024 * 32 && s < 1024 * 1024 * 1024 * 64 => "32G-64G".to_string(),
        s if s >= 1024 * 1024 * 1024 * 64 && s < 1024 * 1024 * 1024 * 128 => "64G-128G".to_string(),
        s if s >= 1024 * 1024 * 1024 * 128 && s < 1024 * 1024 * 1024 * 256 => {
            "128G-256G".to_string()
        }
        s if s >= 1024 * 1024 * 1024 * 256 && s < 1024 * 1024 * 1024 * 512 => {
            "256G-512G".to_string()
        }
        s if s >= 1024 * 1024 * 1024 * 512 && s < 1024 * 1024 * 1024 * 1024 => {
            "512G-1T".to_string()
        }
        _ => "1T+".to_string(),
    };
}

pub fn new_analyze_map() -> DashMap<String, i128> {
    let map = DashMap::<String, i128>::new();
    map.insert("0-1K".to_string(), 0);
    map.insert("1K-8K".to_string(), 0);
    map.insert("8K-16K".to_string(), 0);
    map.insert("16K-32K".to_string(), 0);
    map.insert("32K-64K".to_string(), 0);
    map.insert("64K-128K".to_string(), 0);
    map.insert("128K-256K".to_string(), 0);
    map.insert("256K-512K".to_string(), 0);
    map.insert("512K-1M".to_string(), 0);
    map.insert("1M-8M".to_string(), 0);
    map.insert("8M-16M".to_string(), 0);
    map.insert("16M-32M".to_string(), 0);
    map.insert("32M-64M".to_string(), 0);
    map.insert("32M-64M".to_string(), 0);
    map.insert("64M-128M".to_string(), 0);
    map.insert("128M-256M".to_string(), 0);
    map.insert("256M-512M".to_string(), 0);
    map.insert("512M-1G".to_string(), 0);
    map.insert("1G-8G".to_string(), 0);
    map.insert("8G-16G".to_string(), 0);
    map.insert("16G-32G".to_string(), 0);
    map.insert("32G-64G".to_string(), 0);
    map.insert("32G-64G".to_string(), 0);
    map.insert("64G-128G".to_string(), 0);
    map.insert("128G-256G".to_string(), 0);
    map.insert("256G-512G".to_string(), 0);
    map.insert("512G-1T".to_string(), 0);
    map.insert("1T+".to_string(), 0);
    map
}

pub fn new_analyze_btree() -> BTreeMap<String, i128> {
    let mut map = BTreeMap::<String, i128>::new();
    map.insert("0-1K".to_string(), 0);
    map.insert("1K-8K".to_string(), 0);
    map.insert("8K-16K".to_string(), 0);
    map.insert("16K-32K".to_string(), 0);
    map.insert("32K-64K".to_string(), 0);
    map.insert("64K-128K".to_string(), 0);
    map.insert("128K-256K".to_string(), 0);
    map.insert("256K-512K".to_string(), 0);
    map.insert("512K-1M".to_string(), 0);
    map.insert("1M-8M".to_string(), 0);
    map.insert("8M-16M".to_string(), 0);
    map.insert("16M-32M".to_string(), 0);
    map.insert("32M-64M".to_string(), 0);
    map.insert("32M-64M".to_string(), 0);
    map.insert("64M-128M".to_string(), 0);
    map.insert("128M-256M".to_string(), 0);
    map.insert("256M-512M".to_string(), 0);
    map.insert("512M-1G".to_string(), 0);
    map.insert("1G-8G".to_string(), 0);
    map.insert("8G-16G".to_string(), 0);
    map.insert("16G-32G".to_string(), 0);
    map.insert("32G-64G".to_string(), 0);
    map.insert("32G-64G".to_string(), 0);
    map.insert("64G-128G".to_string(), 0);
    map.insert("128G-256G".to_string(), 0);
    map.insert("256G-512G".to_string(), 0);
    map.insert("512G-1T".to_string(), 0);
    map.insert("1T+".to_string(), 0);
    map
}

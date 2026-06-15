use anyhow::Result;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_yaml::from_str;
use std::env;
use std::fs;
use std::sync::Mutex;
use std::sync::RwLock;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LogLevel {
    OFF,
    ERROR,
    WARN,
    INFO,
    DEBUG,
    TRACE,
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::INFO
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub log_level: LogLevel,
}

impl Config {
    pub fn default() -> Self {
        Self {
            log_level: LogLevel::default(),
        }
    }

    pub fn set_self(&mut self, config: Config) {
        self.log_level = config.log_level;
    }

    pub fn get_config_image(&self) -> Self {
        self.clone()
    }
}

pub fn generate_default_config(path: &str) -> Result<()> {
    let config = Config::default();
    let yml = serde_yaml::to_string(&config)?;
    fs::write(path, yml)?;
    Ok(())
}

static GLOBAL_CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| {
    Mutex::new({
        let global_config = Config::default();
        global_config
    })
});

static CONFIG_FILE_PATH: Lazy<RwLock<String>> = Lazy::new(|| {
    RwLock::new({
        let path = "".to_string();
        path
    })
});

pub fn set_config(path: &str) -> Result<()> {
    let mut current_path = env::current_exe()?.parent().unwrap().to_path_buf();
    current_path.push("config.yml");

    if path.is_empty() {
        if current_path.exists() {
            let contents =
                fs::read_to_string(current_path).expect("Read config file config.yml error!");
            let config = from_str::<Config>(contents.as_str()).expect("Parse config.yml error!");

            let mut conf = GLOBAL_CONFIG
                .try_lock()
                .map_err(|e| anyhow::anyhow!("Lock failed: {}", e))?;
            conf.set_self(config);
        }
        return Ok(());
    }

    let err_str = format!("Read config file {} error!", path);
    let contents = fs::read_to_string(path).expect(err_str.as_str());
    let config = from_str::<Config>(contents.as_str()).expect("Parse config.yml error!");
    let mut conf = GLOBAL_CONFIG
        .try_lock()
        .map_err(|e| anyhow::anyhow!("Lock failed: {}", e))?;
    conf.set_self(config);
    Ok(())
}

pub fn set_config_file_path(path: String) -> Result<()> {
    let mut config_file_path = CONFIG_FILE_PATH
        .try_write()
        .map_err(|e| anyhow::anyhow!("Lock failed: {}", e))?;
    config_file_path.clear();
    config_file_path.push_str(path.as_str());
    Ok(())
}

pub fn get_config_file_path() -> String {
    CONFIG_FILE_PATH.read().unwrap().clone()
}

pub fn get_config() -> Result<Config> {
    let locked_config = GLOBAL_CONFIG
        .try_lock()
        .map_err(|e| anyhow::anyhow!("Lock failed: {}", e))?;
    Ok(locked_config.get_config_image())
}

pub fn get_current_config_yml() -> Result<String> {
    let c = get_config()?;
    let yml = serde_yaml::to_string(&c)?;
    Ok(yml)
}

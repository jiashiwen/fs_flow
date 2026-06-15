// use super::FilePosition;
use crate::{commons::append_line_to_file, models::model_checkpoint::FilePosition};
use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::Write,
    str::FromStr,
    sync::{atomic::AtomicBool, Arc},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListedRecord {
    pub file_num: i32,
    pub key: String,
    pub offset: usize,
    pub line_num: u64,
}

impl FromStr for ListedRecord {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let r = serde_json::from_str::<Self>(s)?;
        Ok(r)
    }
}

impl ListedRecord {
    pub fn save_json_to_file(&self, file: &mut File) -> Result<()> {
        let mut json = serde_json::to_string(self)?;
        json.push_str("\n");
        file.write_all(json.as_bytes())?;
        file.flush()?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Opt {
    PUT,
    REMOVE,
    COMPARE,
    UNKOWN,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecordOption {
    pub source_key: String,
    pub target_key: String,
    pub list_file_path: String,
    pub list_file_position: FilePosition,
    pub option: Opt,
}

impl FromStr for RecordOption {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let r = serde_json::from_str::<Self>(s)?;
        Ok(r)
    }
}

impl RecordOption {
    pub fn handle_error(
        &self,
        stop_mark: Arc<AtomicBool>,
        err_occur: Arc<AtomicBool>,
        append_to: &str,
    ) {
        err_occur.store(true, std::sync::atomic::Ordering::SeqCst);
        stop_mark.store(true, std::sync::atomic::Ordering::SeqCst);
        let _ = self.append_json_to_file(append_to);
    }

    pub fn save_json_to_file(&self, mut file: &File) -> Result<()> {
        let mut json = serde_json::to_string(self)?;
        json.push_str("\n");
        file.write_all(json.as_bytes())?;
        file.flush()?;
        Ok(())
    }

    pub fn append_json_to_file(&self, file_name: &str) -> Result<()> {
        let json = serde_json::to_string(self)?;
        append_line_to_file(file_name, &json)
    }
}

#[cfg(test)]
mod test {
    use super::ListedRecord;
    use std::{fs::OpenOptions, io::Write, path::Path};

    //cargo test checkpoint::record::test::test_error_record -- --nocapture
    #[test]
    fn test_error_record() {
        let file_name = "/tmp/err_dir/error_record";
        let path = Path::new(file_name);
        if let Some(p) = path.parent() {
            std::fs::create_dir_all(p).unwrap();
        };

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file_name)
            .unwrap();

        for i in 0..100 {
            let mut key = "tt/ttt/tttt".to_string();
            key.push_str(i.to_string().as_str());
            let offset = 3214 + i;
            let offset_usize: usize = offset.try_into().unwrap();
            let record = ListedRecord {
                key,
                offset: offset_usize,
                line_num: 1,
                file_num: 0,
            };
            let _ = record.save_json_to_file(&mut file);
        }

        let record1 = ListedRecord {
            key: "test/test1/ttt".to_string(),
            offset: 65,
            line_num: 1,
            file_num: 0,
        };

        let r = record1.save_json_to_file(&mut file);

        log::debug!("r is {:?}", r);

        let record2 = ListedRecord {
            key: "test/test2/tt222".to_string(),
            offset: 77,
            line_num: 1,
            file_num: 0,
        };
        let _ = record2.save_json_to_file(&mut file);

        file.flush().unwrap();
    }
}

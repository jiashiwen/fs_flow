use crate::{
    commons::{
        count_file_bytes, count_file_lines, read_yaml_file, struct_to_yaml_string,
        FileOperationBuilder,
    },
    models::model_checkpoint::CheckPoint,
};
use anyhow::{Context, Result};
use std::{
    fs::File,
    io::{Seek, SeekFrom, Write},
    time::{SystemTime, UNIX_EPOCH},
};

impl CheckPoint {
    pub fn seeked_execute_file(&self) -> Result<File> {
        let mut file =
            File::open(&self.executing_file.path).context(format!("{}:{}", file!(), line!()))?;
        let seek_offset = TryInto::<u64>::try_into(self.executing_file_position.offset)
            .context(format!("{}:{}", file!(), line!()))?;
        file.seek(SeekFrom::Start(seek_offset))
            .context(format!("{}:{}", file!(), line!()))?;
        Ok(file)
    }

    pub fn save_to(&mut self, path: &str) -> Result<()> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .context(format!("{}:{}", file!(), line!()))?;
        self.modify_checkpoint_timestamp = i128::from(now.as_secs());
        self.executing_file.size = count_file_bytes(self.executing_file.path.as_str())
            .context(format!("{}:{}", file!(), line!()))?;
        self.executing_file.total_lines = count_file_lines(self.executing_file.path.as_str())
            .context(format!("{}:{}", file!(), line!()))?;

        let checkpoint_file_builder = FileOperationBuilder::new(path).with_parent_creation();
        let mut file =
            checkpoint_file_builder
                .build_file()
                .context(format!("{}:{}", file!(), line!()))?;

        let constent = struct_to_yaml_string(self).context(format!("{}:{}", file!(), line!()))?;
        file.write_all(constent.as_bytes())
            .context(format!("{}:{}", file!(), line!()))?;
        file.flush().context(format!("{}:{}", file!(), line!()))?;
        Ok(())
    }

    pub fn save_to_file(&mut self, file: &mut File) -> Result<()> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .context(format!("{}:{}", file!(), line!()))?;

        self.modify_checkpoint_timestamp = i128::from(now.as_secs());
        let constent = struct_to_yaml_string(self).context(format!("{}:{}", file!(), line!()))?;
        file.write_all(constent.as_bytes())
            .context(format!("{}:{}", file!(), line!()))?;
        file.flush()?;
        Ok(())
    }
}

pub fn get_task_checkpoint(checkpoint_file: &str) -> Result<CheckPoint> {
    let checkpoint = read_yaml_file::<CheckPoint>(checkpoint_file)?;
    Ok(checkpoint)
}

#[cfg(test)]
mod test {

    use crate::checkpoint::checkpoint::get_task_checkpoint;

    //cargo test checkpoint::checkpoint::test::test_get_task_checkpoint -- --nocapture
    #[test]
    fn test_get_task_checkpoint() {
        println!("get_task_checkpoint");
        let c = get_task_checkpoint("/tmp/meta_dir/checkpoint.yml");
        println!("{:?}", c);
    }
}

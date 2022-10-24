use std::fs;
use std::io::BufWriter;
use std::path::{PathBuf};
// use tokio::{fs, io::BufWriter};
use tonic::Status;
use serde_json;

use crate::config_manager::{Config, ConfigRequestVersion, Empty, ConfigList, ConfigRequestPath};

pub struct FileManager {}

impl FileManager {
    pub async fn create_config(config: &Config) -> Result<(), Status>{
        let file_name = format!("{}.json", config.name);
        let dir_path =  format!("configs/{}", config.name);

        let mut config_path = PathBuf::new();
        config_path.push(&dir_path);

        if config_path.exists() {
            return Err(Status::already_exists("config already exist"));
        }

        fs::create_dir(&config_path)?;

        config_path.push(&file_name);

        let mut file = fs::File::create(&config_path).unwrap();

        let mut writer = BufWriter::new(file);

        serde_json::to_writer_pretty(&mut writer, &config).unwrap();

        Ok(())
    }
}
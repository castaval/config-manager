use std::error::Error;
use std::fs::{self, File};
use std::io::{BufWriter, BufReader};
use std::path::{PathBuf};
// use tokio::{fs, io::BufWriter};
use tonic::{Status, Response};
use serde_json::{self, error};

use crate::config_manager::{Config, ConfigRequest, Empty, ConfigList};

pub struct FileManager {}

impl FileManager {
    pub async fn create_config(config: &Config) -> Result<(), Box<dyn Error>>{
        let file_name = format!("{}_{}.json", config.name, config.version);
        let dir_path =  format!("configs/{}", config.name);

        let mut config_path = PathBuf::new();
        config_path.push(&dir_path);

        if config_path.exists() {
            return Err(Box::new(Status::already_exists("config already exist")));
        }

        fs::create_dir(&config_path)?;

        config_path.push(&file_name);

        let file = fs::File::create(&config_path)?;

        let mut writer = BufWriter::new(file);

        serde_json::to_writer_pretty(&mut writer, &config)?;

        Ok(())
    }

    pub async fn get_config(name: &str) -> Result<Config, Box<dyn Error>> {
        let dir_path = format!("configs/{}", name);
        let mut config_path = PathBuf::new();
        config_path.push(&dir_path);

        if !config_path.exists() {
            return Err(Box::new(Status::not_found("config not found")));
        }

        let config = get_last_config(&dir_path)?;

        Ok(config)
    }

    pub async fn get_all_configs() -> Result<ConfigList, Box<dyn Error>> {
        let mut config_path = PathBuf::new();
        config_path.push("configs");

        let mut config_list = Vec::new();

        for dir in fs::read_dir(config_path)? {
            let dir = dir?;
            
            let config = dir.path();

            if let Some(path) = config.to_str() {
                config_list.push(get_last_config(path)?);
            }
        }

        Ok(ConfigList { configs: config_list })
    }

    pub async fn delete_config(name: &str) -> Result<(), Box<dyn Error>> {
        let dir_path = format!("configs/{}", name);
        let mut config_path = PathBuf::new();
        config_path.push(&dir_path);

        if !config_path.exists() {
            return Err(Box::new(Status::not_found("config not exist")));
        }

        fs::remove_dir_all(config_path)?;

        Ok(())
    }
}

fn get_last_config(name: &str) -> Result<Config, Box<dyn Error>> {
    let mut get_config = Config::default();

    for config in fs::read_dir(name)? {
        let config = config?;

        let file = File::open(config.path())?;
        let reader = BufReader::new(file);

        let json_config: Config = serde_json::from_reader(reader)?;

        if json_config.version > get_config.version {
            get_config = json_config;
        } 
    }

    Ok(get_config)
}
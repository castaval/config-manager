use std::error::Error;
use std::fs::{self, File};
use std::io::{BufWriter, BufReader};
use std::path::{PathBuf};
// use tokio::{fs, io::BufWriter};
use tonic::{Status, Response};
use serde_json::{self, error};

use crate::config_manager::{Config, ConfigList, ConfigInformation};

pub struct FileManager {}

impl FileManager {
    pub async fn create_config(config: &Config) -> Result<(), Box<dyn Error>>{
        let file_name = format!("{}_{}.json", config.service, config.version);
        let dir_path =  format!("configs/{}", config.service);

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
                let config = get_last_config(path)?;
                let config_information =
                    ConfigInformation { service: config.service, data: config.data};
                config_list.push(config_information);
            }
        }

        Ok(ConfigList { configs: config_list })
    }

    pub async fn update_config(new_config: &mut ConfigInformation) -> Result<(), Box<dyn Error>> {
        let dir_path = format!("configs/{}", &new_config.service);
        let mut config_path = PathBuf::new();
        config_path.push(&dir_path);

        if !config_path.exists() {
            return Err(Box::new(Status::not_found("config not found")));
        }

        let mut old_config = get_last_config(&dir_path)?;

        if old_config.used {
            return Err(Box::new(Status::aborted("This config is using")));
        }

        old_config.data.extend(new_config.data.clone());

        let updated_config = Config {
            version: old_config.version + 1,
            service: old_config.service,
            data: old_config.data,
            used: false,
        };

        let file_name = format!("{}_{}.json", updated_config.service, updated_config.version);
        config_path.push(&file_name);

        let file = fs::File::create(&config_path)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &updated_config)?;

        Ok(())
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

    pub async fn delete_config_version(name: &str, version: u32) -> Result<(), Box<dyn Error>> {
        let dir_path = format!("configs/{}", name);
        let mut config_path = PathBuf::new();
        config_path.push(&dir_path);

        for conf in fs::read_dir(config_path)? {
            let conf = conf?;

            let file = File::open(conf.path())?;
            let reader = BufReader::new(file);

            let config: Config = serde_json::from_reader(reader)?;

            if config.version == version {
                if config.used {
                    return Err(Box::new(Status::aborted("This config is using")));
                }

                fs::remove_file(conf.path())?;
                break;
            }
        }

        Ok(())
    }

    pub async fn use_config_version(name: &str, version: u32) -> Result<(), Box<dyn Error>> {
        let dir_path = format!("configs/{}", name);
        let mut config_path = PathBuf::new();
        config_path.push(&dir_path);


        for conf in fs::read_dir(&config_path)? {
            let conf = conf?;

            let file = File::open(conf.path())?;
            let reader = BufReader::new(file);

            let config: Config = serde_json::from_reader(reader)?;

            if config.used {
                let config_not_used = Config {
                    service: config.service,
                    version: config.version,
                    data: config.data,
                    used: false,
                };

                fs::remove_file(conf.path())?;

                let conf_path = conf.path();

                let file = fs::File::create(&conf_path)?;
                let mut writer = BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &config_not_used)?;

                break;
            }
        }

        for conf in fs::read_dir(&config_path)? {
            let conf = conf?;

            let file = File::open(conf.path())?;
            let reader = BufReader::new(file);

            let config: Config = serde_json::from_reader(reader)?;

            if config.version == version {
                let config_used = Config {
                    service: config.service,
                    version: config.version,
                    data: config.data,
                    used: true,
                };

                fs::remove_file(conf.path())?;

                let conf_path = conf.path();

                let file = fs::File::create(&conf_path)?;
                let mut writer = BufWriter::new(file);
                serde_json::to_writer_pretty(&mut writer, &config_used)?;
            }
        }

        Ok(())
    }
}

fn get_last_config(path: &str) -> Result<Config, Box<dyn Error>> {
    let mut get_config = Config::default();

    for config in fs::read_dir(path)? {
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
use std::error::Error;
use std::fs::File;

use tonic::{transport::Server, Request, Response, Status};

use config_manager::config_manager_server::{ConfigManagerServer, ConfigManager};
use config_manager::{Config, Empty, ConfigList, ConfigRequest, ResponseReply, ConfigRequestVersion};

use file_manager::FileManager;

mod file_manager;

pub mod config_manager {
    tonic::include_proto!("configmanager");
}

#[derive(Debug, Default)]
pub struct Manager {}

#[tonic::async_trait]
impl ConfigManager for Manager {
    async fn create(
        &self,
        request: Request<Config>,
    ) -> Result<Response<ResponseReply>, Status> {
        println!("Got a create request {:?}", request);

        let config = config_manager::Config {
            version: request.get_ref().version,
            name: request.get_ref().name.clone(),
            data: request.get_ref().data.clone(),
            used: request.get_ref().used.clone(),
        };

        match FileManager::create_config(&config).await {
            Ok(_) => (),
            Err(e) => return Err(Status::cancelled(format!("{}", e))),
        }

        let response = config_manager::ResponseReply {
            status: String::from("Success"),
        };

        Ok(Response::new(response))
    }

    async fn get(
         &self,
         request: Request<ConfigRequest>,
    ) -> Result<Response<Config>, Status> {
        println!("Got a get request {:?}", request);

        let config = match FileManager::get_config(&request.get_ref().name).await {
            Ok(config) => config,
            Err(e) => return Err(Status::not_found(format!("{}", e))),
        };

        let response = config;

        Ok(Response::new(response))
    }

    async fn get_all(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<ConfigList>, Status> {
        println!("Got a get all request {:?}", request);

        let configs = match FileManager::get_all_configs().await {
            Ok(configs) => configs,
            Err(e) => return Err(Status::not_found(format!("{}", e))),
        };

        let response = configs;

        Ok(Response::new(response))
    }

    async fn update(
        &self,
        request: Request<Config>,
    ) -> Result<Response<ResponseReply>, Status> {
        let mut request = request; 

        println!("Got a delete request {:?}", request);

        match FileManager::update_config(request.get_mut()).await {
            Ok(_) => (),
            Err(e) => return Err(Status::cancelled(format!("{}", e))),
        }

        let response = config_manager::ResponseReply {
            status: String::from("Config was updated"),
        };

        Ok(Response::new(response))
    }

    async fn delete(
        &self,
        request: Request<ConfigRequest>,
    ) -> Result<Response<ResponseReply>, Status> {
        println!("Got a delete request {:?}", request);

        match FileManager::delete_config(&request.get_ref().name).await {
            Ok(_) => (),
            Err(e) => return Err(Status::not_found(format!("{}", e))),
        };

        let response = config_manager::ResponseReply {
            status: String::from("Config was deleted"),
        };

        Ok(Response::new(response))
    }

    async fn delete_version(
        &self,
        request: Request<ConfigRequestVersion>,
    ) -> Result<Response<ResponseReply>, Status> {
        println!("Got ad delete version request {:?}", request);

        match FileManager::delete_config_version(&request.get_ref().name, request.get_ref().version).await {
            Ok(_) => (),
            Err(e) => return Err(Status::not_found(format!("{}", e))),
        }

        let response = config_manager::ResponseReply {
            status: String::from("Config version was deleted"),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "[::1]:50051".parse()?;
    let manager = Manager::default();

    Server::builder()
        .add_service(ConfigManagerServer::new(manager))
        .serve(addr)
        .await?;

    Ok(())
}


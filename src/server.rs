use tonic::{transport::Server, Request, Response, Status};

use config_manager::config_manager_server::{ConfigManagerServer, ConfigManager};
use config_manager::{Config, ConfigRequestVersion, Empty, ConfigList, ConfigRequestPath, ResponseReply};

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
            version: request.get_ref().version.clone(),
            name: request.get_ref().name.clone(),
            data: request.get_ref().data.clone(),
            used: request.get_ref().used.clone(),
        };

        FileManager::create_config(&config).await?;

        let response = config_manager::ResponseReply {
            status: String::from("Success"),
        };

        Ok(Response::new(response))
    }

    async fn get(
         &self,
         request: Request<ConfigRequestVersion>,
    ) -> Result<Response<Config>, Status> {
        unimplemented!()
    }

    async fn get_all(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<ConfigList>, Status> {
        unimplemented!()
    }

    async fn update(
        &self,
        request: Request<Config>,
    ) -> Result<Response<Config>, Status> {
        unimplemented!()
    }

    async fn delete(
        &self,
        request: Request<ConfigRequestPath>,
    ) -> Result<Response<Empty>, Status> {
        unimplemented!()
    }

    async fn delete_version(
        &self,
        request: Request<ConfigRequestVersion>,
    ) -> Result<Response<Empty>, Status> {
        unimplemented!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let manager = Manager::default();

    Server::builder()
        .add_service(ConfigManagerServer::new(manager))
        .serve(addr)
        .await?;

    Ok(())
}


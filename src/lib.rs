use std::{error::Error, collections::HashMap};

use config_manager::{config_manager_client::ConfigManagerClient, ConfigInformation, ResponseReply, RequestService, ResponseGet, Empty, ConfigList, RequestServiceVersion};
use tonic::{Request, Response, Status, transport::Channel, codegen::http::{response, version}, service};

pub mod config_manager {
    tonic::include_proto!("configmanager");
}

pub struct ConfigClient {
    client: ConfigManagerClient<Channel>,
}

impl ConfigClient {
    pub async fn new(address: &str) -> Result<ConfigClient, Box<dyn Error>> {
        let adr = String::from(address);
        let client = ConfigManagerClient::connect(adr).await?;

        Ok(ConfigClient { client })
    }

    pub async fn create_config(&mut self, service_name: &str, data: HashMap<String, String>) -> Result<Response<ResponseReply>, Status> {
        let request = Request::new(ConfigInformation {
            service: service_name.to_string(),
            data,
        });

        let response = self.client.create(request).await?;

        Ok(response)
    }

    pub async fn get(&mut self, service_name: &str) -> Result<Response<ResponseGet>, Status> {
        let request = Request::new(RequestService {
            service: service_name.to_string(),
        });

        let response = self.client.get(request).await?;

        Ok(response)
    }

    pub async fn get_version(&mut self, service_name: &str, version: u32) -> Result<Response<ResponseGet>, Status> {
        let request = Request::new(RequestServiceVersion {
            service: service_name.to_string(),
            version,
        });

        let response = self.client.get_version(request).await?;

        Ok(response)
    }

    pub async fn get_all(&mut self) -> Result<Response<ConfigList>, Status> {
        let request = Request::new(Empty {});

        let response = self.client.get_all(request).await?;

        Ok(response)
    }

    pub async fn update(&mut self, service_name: &str, data: HashMap<String, String>) -> Result<Response<ResponseReply>, Status> {
        let request = Request::new(ConfigInformation {
            service: service_name.to_string(),
            data,
        });

        let response = self.client.update(request).await?;

        Ok(response)
    }

    pub async fn delete(&mut self, service_name: &str) -> Result<Response<ResponseReply>, Status> {
        let request = Request::new(RequestService {
            service: service_name.to_string(),
        });

        let response = self.client.delete(request).await?;

        Ok(response)
    }

    pub async fn delete_version(&mut self, service_name: &str, version: u32) -> Result<Response<ResponseReply>, Status> {
        let request = Request::new(RequestServiceVersion {
            service: service_name.to_string(),
            version
        });

        let response = self.client.delete_version(request).await?;

        Ok(response)
    }

    pub async fn use_config(&mut self, service_name: &str, version: u32) -> Result<Response<ResponseReply>, Status> {
        let request = Request::new(RequestServiceVersion {
            service: service_name.to_string(),
            version
        });

        let response = self.client.use_config(request).await?;

        Ok(response)
    }


}
use std::{error::Error, collections::HashMap};

use config_manager::{ConfigClient};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = ConfigClient::new("http://0.0.0.0:3030").await?;

    let mut map1: HashMap<String, String> = HashMap::new();
    map1.insert(String::from("password"), String::from("test"));
    let created_config = client.create_config("test", map1).await?;
    println!("{:?}", created_config);

    let getted_config = client.get("test").await?;
    println!("{:?}", getted_config);

    let getted_version_config = client.get_version("test", 1).await?;
    println!("{:?}", getted_version_config);

    let getted_all = client.get_all().await?;
    println!("{:?}", getted_all);

    let mut map2: HashMap<String, String> = HashMap::new();
    map2.insert(String::from("login"), String::from("test"));
    let updated_config = client.update("test", map2).await?;
    println!("{:?}", updated_config);

    let used_config = client.use_config("test", 1).await?;
    println!("{:?}", used_config);

    let deleted_version_config = client.delete_version("service_name", 1).await?;
    println!("{:?}", deleted_version_config);

    let used_config = client.use_config("test", 1).await?;
    println!("{:?}", used_config);

    let deleted_config = client.delete("test").await?;
    println!("{:?}", deleted_config);

    Ok(())
}
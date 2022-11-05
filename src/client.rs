use std::{error::Error, collections::HashMap};

use config_manager::{ConfigClient};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = ConfigClient::new("http://0.0.0.0:3030").await?;

    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(String::from("password"), String::from("test"));
    client.create_config("test", map).await?;

    let response = client.use_config("test", 1).await?;
    assert_eq!(response.status, "Config was used");

    client.use_config("test", 1).await?;
    Ok(())
}
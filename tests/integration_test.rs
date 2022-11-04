use std::{error::Error, collections::HashMap};

use config_manager::ConfigClient;


#[tokio::test]
async fn test_create_config() -> Result<(), Box<dyn Error>> {
    let mut client = ConfigClient::new("http://[::1]:50051").await?;

    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(String::from("password"), String::from("test"));

    let response = client.create_config("test", map).await?;

    assert_eq!(response.status, String::from("Success"));
    client.delete("test").await?;
    Ok(())
}

#[tokio::test]
async fn test_get() -> Result<(), Box<dyn Error>> {
    let mut client = ConfigClient::new("http://[::1]:50051").await?;

    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(String::from("password"), String::from("test"));
    client.create_config("test", map.clone()).await?;

    let response = client.get("test").await?;
    assert_eq!(response.data, map);

    client.delete("test").await?;
    Ok(())
}

#[tokio::test]
async fn test_get_version() -> Result<(), Box<dyn Error>> {
    let mut client = ConfigClient::new("http://[::1]:50051").await?;

    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(String::from("password"), String::from("test"));
    client.create_config("test", map.clone()).await?;

    let response = client.get_version("test", 1).await?;
    assert_eq!(response.data, map);

    client.delete("test").await?;
    Ok(())
}

#[tokio::test]
async fn test_get_all_configs() -> Result<(), Box<dyn Error>> {
    let mut client = ConfigClient::new("http://[::1]:50051").await?;

    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(String::from("password"), String::from("test"));
    client.create_config("test", map).await?;

    let response = client.get_all().await?;
    assert_ne!(response.configs.len(), 0);

    client.delete("test").await?;
    Ok(())
}

#[tokio::test]
async fn test_update() -> Result<(), Box<dyn Error>> {
    let mut client = ConfigClient::new("http://[::1]:50051").await?;

    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(String::from("password"), String::from("test"));
    client.create_config("test", map.clone()).await?;

    let response = client.update("test", map).await?;
    assert_eq!(response.status, "Config was updated");

    client.delete("test").await?;
    Ok(())
}

#[tokio::test]
async fn test_delete() -> Result<(), Box<dyn Error>> {
    let mut client = ConfigClient::new("http://[::1]:50051").await?;

    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(String::from("password"), String::from("test"));
    client.create_config("test", map).await?;

    let response = client.delete("test").await?;
    assert_eq!(response.status, "Config was deleted");

    Ok(())
}

#[tokio::test]
async fn test_delete_version() -> Result<(), Box<dyn Error>> {
    let mut client = ConfigClient::new("http://[::1]:50051").await?;

    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(String::from("password"), String::from("test"));
    client.create_config("test", map).await?;

    let response = client.delete_version("test", 1).await?;
    assert_eq!(response.status, "Config version was deleted");

    client.delete("test").await?;
    Ok(())
}

#[tokio::test]
async fn test_use_config() -> Result<(), Box<dyn Error>> {
    let mut client = ConfigClient::new("http://[::1]:50051").await?;

    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(String::from("password"), String::from("test"));
    client.create_config("test", map).await?;

    let response = client.use_config("test", 1).await?;
    assert_eq!(response.status, "Config was used");

    client.use_config("test", 1).await?;
    client.delete("test").await?;
    Ok(())
}


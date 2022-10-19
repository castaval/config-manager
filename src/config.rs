use rocket::serde::{Serialize, Deserialize, json::Json};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Config {
    service_name: String,
    data: Vec<String>
}

impl Config {
    pub fn new(service_name: String, data: Vec<String>) -> Config {
        Config { service_name, data }
    }
}
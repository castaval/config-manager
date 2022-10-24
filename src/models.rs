use std::collections::HashMap;

use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Config {
    pub service_name: String,
    pub data: Vec<Option<HashMap<String, String>>>,
    pub used: bool,
}
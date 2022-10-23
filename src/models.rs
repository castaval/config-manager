use std::collections::HashMap;

use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Config {
    pub service_name: String,
    pub data: Vec<Option<HashMap<String, String>>>,
    pub used: bool,
}

impl Config {
    // pub fn new(service_name: String, data: Vec<Option<Atribute>>, used: bool) -> Config {
    //     Config { service_name, data, used }
    // }

    // pub fn get_service_name(&self) -> &str {
    //     &self.service_name
    // }

    // pub fn get_data(&self, key: &str) -> &str {
    //     &self.data.get(key).unwrap()
    // }

    // pub fn is_used(&self) -> &bool {
    //     &self.used
    // }

    // pub fn rename(&self, new_name: String) {
    //     self.service_name = new_name;
    // }

    // pub fn change_using(&self, new_state: bool) {
    //     self.used = new_state;
    // }

}
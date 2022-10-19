#[macro_use] extern crate rocket;
use rocket::{response::content, serde::json::Json};
use config::Config;
mod config;

#[get("/")]
fn index() -> &'static str {
    "Hello, this is a config-manager!"
}

#[post("/config", data = "<input_config>")]
fn create_config(input_config: Json<Config>) -> Json<&'static str> {
    println!("{}", input_config);


    Json("{ 'hi : 'world' }")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, create_config])
}
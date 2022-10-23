
use rocket::tokio::io::AsyncWriteExt;
use rocket::{Request, Response, serde};
use rocket::serde::json::Json;
use rocket::tokio::fs::{File, write};
use super::models::{Config};


#[get("/")]
pub fn index() -> &'static str {
    "Hello, this is a config-manager!"
}

#[get("/config?service=<service_name>", format = "json")]
pub fn get_config() {

}

#[post("/config", format = "json", data = "<input_config>")]
pub async fn create_config<'a>(input_config: Json<Config>) -> Json<&'a str> {
    let mut file = File::create(&input_config.service_name).await.unwrap();

    let serialized_config = serde_json::to_string(&input_config.data).unwrap();

    file.write(serialized_config.as_bytes()).await.unwrap();
    
    Json("Success")
}

#[delete]
pub async fn delete_config()

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

#[catch(422)]
pub fn unprocessable_entity(req: &Request) -> String {
    String::from("Unprocessable entity!")
}
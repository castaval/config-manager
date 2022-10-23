#[macro_use] extern crate rocket;
use handlers::{index, create_config, not_found, unprocessable_entity};

mod models;
mod handlers;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, create_config])
        .register("/", catchers![not_found, unprocessable_entity])
}
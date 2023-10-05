#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket::error::LaunchError;
use rocket_contrib::serve::StaticFiles;
use rocket::{Config, config::Environment};

#[path = "./modules/controller/router.rs"]
mod router;



fn rocket() -> LaunchError{
    let config = Config::build(Environment::Development)
        .address("0.0.0.0")
        .port(8000)
        .finalize()
        .unwrap();


    rocket::custom(config)
    .mount("/", router::routes())
    .mount("/style", StaticFiles::from("src/modules/templates/style"))
    .mount("/script", StaticFiles::from("src/modules/templates/script"))
    .launch()
}

fn main() {
    rocket();
}
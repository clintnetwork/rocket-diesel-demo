#![feature(plugin, custom_derive, custom_attribute)]
#![plugin(rocket_codegen)]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

pub mod schema;
pub mod models;
pub mod controllers;
pub mod db;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_infer_schema;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

use controllers::origin::*;

fn main() {
    rocket::ignite()
        .mount("/", routes![world, hello])
        .manage(db::init_pool())
        .launch();
}

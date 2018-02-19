#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;

extern crate clap;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;
extern crate url;

pub mod schema;
pub mod models;
pub mod controllers;
pub mod db;
pub mod config;
pub mod error;

use std::io::stdout;
pub use self::config::{Config, CFG_DEFAULT_PATH};
pub use self::error::{Error, Result};

use clap::{App, Arg, SubCommand};
use controllers::origin;

embed_migrations!("migrations");

fn main() {
    let matches = app().get_matches();
    let config = match matches.value_of("config") {
        Some(cfg_path) => Config::from_file(cfg_path).unwrap(),
        None => Config::from_file(CFG_DEFAULT_PATH).unwrap_or(Config::default()),
    };
    let conn = db::init_pool(config.db);
    let handle = conn.get().expect("Couldn't get a db handle");
    embedded_migrations::run_with_output(&*handle, &mut stdout()).unwrap();
    rocket::ignite()
        .mount("/", origin::routes())
        .manage(conn)
        .launch();
}

fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("Rocket Diesel Demo")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .subcommand(SubCommand::with_name("start").about("Start Mjolnir"))
}
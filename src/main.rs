#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;

extern crate clap;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_infer_schema;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate failure;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate toml;
extern crate url;

pub mod error;
pub mod schema;
pub mod models;
pub mod controllers;
pub mod db;
pub mod config;

use clap::{Arg, App, SubCommand};
use config::Config;
use controllers::origin;
use std::io::stdout;
use std::process;
use error::Result;

embed_migrations!("migrations");

fn main() {
    let matches = app().get_matches();
    let config = match config_from_args(&matches) {
        Ok(result) => result,
        Err(e) => process::exit(1),
    };
    let conn = db::init_pool(config.db);
    let handle = conn.get().expect("Couldn't get a db handle");
    embedded_migrations::run_with_output(&*handle, &mut stdout()).unwrap();
    rocket::ignite()
        .mount("/", origin::routes())
        .manage(conn)
        .launch();
}

fn app<'a, 'b>() -> clap::App<'a, 'b> {
    App::new("Mjolnir")
        .subcommand(SubCommand::with_name("start")
        .about("Start Mjolnir")
            .arg(Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true)))
}

fn config_from_args(matches: &clap::ArgMatches) -> Result<Config> {
    let cmd = matches.subcommand_name().unwrap();
    let args = matches.subcommand_matches(cmd).unwrap();
    let config = match args.value_of("config") {
        Some(cfg_path) => Config::from_file(cfg_path)?,
        None => Config::default(),
    };
    Ok(config)
}


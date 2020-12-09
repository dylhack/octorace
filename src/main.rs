//noinspection RsMainFunctionNotFound
#[macro_use]
extern crate rocket;

mod api;
mod config;
mod db;
mod macros;
mod models;
mod oauth;

use crate::api::guilds::*;
use crate::api::user::*;
use crate::oauth::create_oauth_client;
use crate::oauth::routes::*;

use crate::config::Config;
use rocket::routes;
use rocket_contrib::serve::StaticFiles;
use std::path::{PathBuf, Path};
use rocket::response::NamedFile;

#[get("/<_file..>")]
async fn get_react_guild(_file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new(crate_relative!("/web/build/index.html"))).await.ok()
}

#[launch]
async fn rocket() -> rocket::Rocket {
    let config = Config::new();

    if config.client_secret.is_empty() {
        println!("Please fill in config.yml");
    }

    let oauth_client = create_oauth_client(config.clone());

    println!("Starting server..");

    rocket::ignite()
        .manage(oauth_client)
        .manage(db::pool::pool().await)
        .manage(config.clone())
        .mount("/", StaticFiles::from(crate_relative!("/web/build")))
        .mount("/oauth", routes![oauth_main, oauth_callback])
        .mount("/api", routes![get_user, get_guilds])
        .mount("/guild", routes![get_react_guild])
}

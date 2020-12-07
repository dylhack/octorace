//noinspection RsMainFunctionNotFound
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate rocket;

mod api;
mod db;
mod macros;
mod models;
mod oauth;
mod schemas;

use crate::api::guilds::*;
use crate::api::user::*;
use crate::oauth::create_oauth_client;
use crate::oauth::routes::*;

use rocket::routes;
use rocket_contrib::serve::StaticFiles;

embed_migrations!("migrations");

#[launch]
fn rocket() -> rocket::Rocket {
    let oauth_client = create_oauth_client();

    embedded_migrations::run(&db::pool::pg_connection()).expect("expected successful migration");

    rocket::ignite()
        .manage(oauth_client)
        .manage(db::pool::pool())
        // .mount("/", StaticFiles::from(crate_relative!("/public")))
        .mount("/oauth", routes![oauth_main, oauth_callback])
        .mount("/api", routes![get_user, get_guilds])
}

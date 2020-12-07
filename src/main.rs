#![feature(decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod api;
mod bot;
mod db;
mod macros;
mod models;
mod oauth;
mod schemas;

use crate::api::user::*;
use crate::bot::start_bot;
use crate::oauth::create_oauth_client;
use crate::oauth::routes::*;

use rocket::routes;
use rocket_contrib::serve::StaticFiles;

embed_migrations!("migrations");

#[tokio::main]
async fn main() {
    let oauth_client = create_oauth_client();

    tokio::spawn(async move {
        start_bot().await;
    });

    embedded_migrations::run(&db::pool::pg_connection()).expect("expected successful migration");

    rocket::ignite()
        .manage(oauth_client)
        .manage(db::pool::pool())
        .mount("/", StaticFiles::from(crate_relative!("/public")))
        .mount("/oauth", routes![oauth_main, oauth_callback])
        .mount("/api", routes![get_user, get_user_contributions])
        .launch();
}

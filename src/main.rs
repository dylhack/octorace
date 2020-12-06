#![feature(decl_macro)]

mod api;
mod bot;
mod macros;
mod models;
mod oauth;

use crate::api::user::*;
use crate::bot::start_bot;
use crate::oauth::create_oauth_client;
use crate::oauth::routes::*;

use rocket::get;
use rocket::http::Cookies;
use rocket::routes;
use rocket_contrib::serve::StaticFiles;
use tokio::macros::*;

#[tokio::main]
async fn main() {
    let client = create_oauth_client();

    tokio::spawn(async move {
        start_bot().await;
    });

    rocket::ignite()
        .manage(client)
        .mount("/", StaticFiles::from(crate_relative!("/public")))
        .mount("/oauth", routes![oauth_main, oauth_callback])
        .mount("/api", routes![get_user, get_user_contributions])
        .launch();
}

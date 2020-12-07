use crate::api::models::DiscordGuild;
use crate::api::{json, ApiResponse};
use crate::db::guard::DbConn;
use crate::diesel::RunQueryDsl;
use crate::models::ApiGuild;
use crate::oauth::oauth_request;
use crate::schemas::diesel::guilds;
use diesel::{ExpressionMethods, PgConnection, QueryDsl};
use rocket::get;
use rocket::http::{CookieJar, Status};
use std::thread;

#[get("/guilds")]
pub async fn get_guilds(jar: &CookieJar<'_>, db: DbConn) -> ApiResponse {
    let token = jar.get("discord_token");
    return match token {
        Some(token) => match get_api_guilds(token.value().to_string(), &db).await {
            Some(guilds) => ApiResponse {
                json: json!(&guilds),
                status: Status::Ok,
            },
            None => ApiResponse {
                json: json!({"Error": "User does not have github connected"}),
                status: Status::BadRequest,
            },
        },
        None => ApiResponse {
            json: json!({"Error": "forbidden"}),
            status: Status::Forbidden,
        },
    };
}

pub async fn get_api_guilds(token: String, db: &DbConn) -> Option<Vec<ApiGuild>> {
    let discord_guilds: Vec<DiscordGuild> = oauth_request("users/@me/guilds", token.clone())
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let mut api_guilds: Vec<ApiGuild> = vec![];
    for guild in discord_guilds {
        let res: i64 = guilds::table
            .filter(guilds::guild_id.eq(guild.id.parse::<i64>().unwrap()))
            .count()
            .get_result(&db.0)
            .unwrap();
        if res > 1 {
            let icon = {
                if let Some(icon) = guild.icon {
                    format!("https://cdn.discordapp.com/icons/{}/{}.png", guild.id, icon)
                } else {
                    "https://cdn.discordapp.com/attachments/723255066898858055/785526045884809256/Itky1.jpg".to_string()
                }
            };
            api_guilds.push(ApiGuild {
                name: guild.name,
                id: guild.id.parse().unwrap(),
                icon_url: icon,
                profiles: res,
            })
        }
    }
    if !api_guilds.is_empty() {
        Some(api_guilds)
    } else {
        None
    }
}

use crate::api::models::DiscordGuild;
use crate::api::{json, ApiResponse};
use crate::db::guard::DbConn;
use crate::diesel::RunQueryDsl;
use crate::models::{ApiGuild, ApiProfile};
use crate::oauth::oauth_request;
use crate::schemas::diesel::{guilds, connections};
use crate::schemas::diesel::users;
use diesel::{ExpressionMethods, QueryDsl, JoinOnDsl, sql_query};
use rocket::get;
use rocket::http::{CookieJar, Status};

#[derive(Queryable)]
pub struct ProfileJoined {

}

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
                json: json!({"Error": "User is not in any valid guilds"}),
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

            let profiles: Vec<ApiProfile> = sql_query(format!("
                SELECT * FROM octorace.guilds
                    INNER JOIN octorace.users u on u.discord_id = guilds.discord_id
                    INNER JOIN octorace.connections c on u.discord_id = c.discord_id
                WHERE guilds.guild_id = {}", guild.id)).load(&db.0).unwrap();

            api_guilds.push(ApiGuild {
                name: guild.name,
                id: guild.id,
                icon_url: icon,
                profiles,
            })
        }
    }
    if !api_guilds.is_empty() {
        Some(api_guilds)
    } else {
        None
    }
}

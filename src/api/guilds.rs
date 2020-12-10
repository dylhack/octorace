use crate::api::models::DiscordGuild;
use crate::api::{json, ApiResponse};
use crate::db::guard::DbConn;
use crate::db::pool::Pool;
use crate::models::{ApiGuild, ApiProfile};
use crate::oauth::oauth_request;
use rocket::get;
use rocket::http::{CookieJar, Status};

#[get("/guilds")]
pub async fn get_guilds(jar: &CookieJar<'_>, db: DbConn<'_>) -> ApiResponse {
    let token = jar.get_private("discord_token");
    jar.get_private("discord_token")
        .map(|crumb| format!("Token: {}", crumb.value()));
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

pub async fn get_api_guilds(token: String, pool: &Pool) -> Option<Vec<ApiGuild>> {
    let discord_guilds: Vec<DiscordGuild> = oauth_request("users/@me/guilds", token.clone())
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let mut api_guilds: Vec<ApiGuild> = vec![];
    for guild in discord_guilds {
        let res = sqlx::query!(
            "SELECT count(*) FROM octorace.guilds WHERE guild_id = $1",
            guild.id.parse::<i64>().unwrap()
        )
        .fetch_one(pool)
        .await
        .unwrap();
        if res.count.unwrap() > 1 {
            let icon = {
                if let Some(icon) = guild.icon {
                    format!("https://cdn.discordapp.com/icons/{}/{}.png", guild.id, icon)
                } else {
                    "https://cdn.discordapp.com/attachments/723255066898858055/785526045884809256/Itky1.jpg".to_string()
                }
            };

            api_guilds.push(ApiGuild {
                name: guild.name.clone(),
                id: guild.id.clone(),
                icon_url: icon,
                profiles: get_profiles(&guild.id.parse().unwrap(), pool).await,
            })
        }
    }

    if !api_guilds.is_empty() {
        Some(api_guilds)
    } else {
        None
    }
}

pub async fn get_profiles(guild_id: &i64, pool: &Pool) -> Vec<ApiProfile> {
    sqlx::query_as!(
        ApiProfile,
        "
                SELECT tag, contributions, avatar_url, github FROM octorace.guilds
                    INNER JOIN octorace.users u on u.discord_id = guilds.discord_id
                    INNER JOIN octorace.connections c on u.discord_id = c.discord_id
                WHERE guilds.guild_id = $1",
        guild_id
    )
    .fetch_all(pool)
    .await
    .unwrap()
}

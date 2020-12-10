use crate::api::models::{DiscordGuild, DiscordUser};
use crate::api::{json, ApiResponse};
use crate::config::Config;
use crate::db::guard::DbConn;
use crate::db::pool::Pool;
use crate::models::ApiProfile;
use crate::oauth::oauth_request;
use chrono::{Duration, Utc};
use graphql_client::*;
use reqwest::Client;
use rocket::get;
use rocket::http::{CookieJar, Status};

#[derive(Debug)]
pub struct UserJoined {
    pub discord_id: i64,
    pub contributions: i32,
    pub github: String,
    pub tag: String,
    pub avatar_url: String,
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/api/schemas/schema.graphql",
    query_path = "src/api/schemas/github_contrib_query.graphql",
    response_derives = "Debug"
)]
pub struct GithubReturn;

#[get("/user")]
pub async fn get_user(jar: &CookieJar<'_>, db: DbConn<'_>) -> ApiResponse {
    let token = jar.get_private("discord_token");
    return match token {
        Some(token) => match get_api_user(token.value().to_string(), &db, jar).await {
            Some(user) => ApiResponse {
                json: json!(&user),
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

pub async fn get_api_user(_token: String, pool: &Pool, jar: &CookieJar<'_>) -> Option<ApiProfile> {
    let user_cookie = jar.get_private("discord_id").unwrap();
    let user_id = user_cookie.value();
    let data: Vec<UserJoined> = sqlx::query_as!(
        UserJoined,
        "SELECT users.discord_id, contributions, github, tag, avatar_url \
    FROM octorace.users \
    INNER JOIN octorace.connections c on users.discord_id = c.discord_id \
    WHERE users.discord_id = $1",
        user_id.parse::<i64>().unwrap()
    )
    .fetch_all(pool)
    .await
    .unwrap();

    if data.is_empty() {
        return None;
    }

    if let Some(db_user) = data.get(0) {
        Some(ApiProfile {
            tag: db_user.tag.clone(),
            github: db_user.github.clone(),
            avatar_url: db_user.avatar_url.clone(),
            contributions: db_user.contributions,
        })
    } else {
        None
    }
}

pub async fn get_contributions(username: String, config: &Config) -> i32 {
    let q = GithubReturn::build_query(github_return::Variables {
        login: username.clone(),
    });

    let client = Client::builder()
        .user_agent("graphql-rust/0.9.0")
        .build()
        .unwrap();

    let res = client
        .post("https://api.github.com/graphql")
        .bearer_auth(config.github_key.clone())
        .json(&q)
        .send()
        .await
        .expect("Unable to query github");

    let data: Response<github_return::ResponseData> = res.json().await.unwrap();

    data.data
        .unwrap()
        .user
        .unwrap()
        .contributions_collection
        .contribution_calendar
        .total_contributions as i32
}

pub async fn make_new_user(user: UserJoined, me: &DiscordUser, pool: &Pool) {
    let mut time = Utc::now() + Duration::minutes(5);
    sqlx::query!(
        "
        INSERT INTO octorace.users (discord_id, contributions, expires, tag, avatar_url)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (discord_id)
                DO NOTHING",
        user.discord_id,
        user.contributions,
        time.naive_utc(),
        format!("{}#{}", me.username, me.discriminator),
        format!(
            "https://cdn.discordapp.com/avatars/{}/{}.png",
            me.id, me.avatar
        )
    )
    .execute(pool)
    .await
    .expect("Unable to insert");

    time = Utc::now() + Duration::days(1);
    sqlx::query!(
        "INSERT INTO octorace.connections (discord_id, github, expires) VALUES ($1, $2, $3)",
        user.discord_id,
        user.github,
        time.naive_utc(),
    )
    .execute(pool)
    .await
    .expect("Unable to insert");
}

pub async fn add_user_guilds(token: String, pool: &Pool, user_id: i64) {
    let discord_guilds: Vec<DiscordGuild> = oauth_request("users/@me/guilds", token)
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    for guild in discord_guilds {
        sqlx::query!(
            "\
            INSERT INTO octorace.guilds (discord_id, guild_id) \
            VALUES ($1, $2) \
            ON CONFLICT DO NOTHING",
            user_id,
            guild.id.parse::<i64>().unwrap()
        )
        .execute(pool)
        .await
        .expect("Unable to insert");
    }
}

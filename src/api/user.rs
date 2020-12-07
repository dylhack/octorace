use crate::api::models::{ApiUserConnection, DiscordGuild, DiscordUser};
use crate::api::{json, ApiResponse};
use crate::db::guard::DbConn;
use crate::models::{ApiActivity, ApiProfile};
use crate::oauth::oauth_request;
use crate::schemas::diesel::connections;
use crate::schemas::diesel::guilds;
use crate::schemas::diesel::users;
use crate::schemas::{NewConnection, NewGuild, NewUser};
use chrono::{Duration, Utc};
use diesel::{
    BoolExpressionMethods, ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl,
};
use reqwest::blocking;
use rocket::get;
use rocket::http::{CookieJar, Status};

#[derive(Debug, Queryable)]
pub struct UserJoined {
    pub discord_id: i64,
    pub contributions: i32,
    pub github: String,
}

#[get("/user")]
pub async fn get_user(jar: &CookieJar<'_>, db: DbConn) -> ApiResponse {
    let token = jar.get("discord_token");
    return match token {
        Some(token) => match get_api_user(token.value().to_string(), &db).await {
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

pub async fn get_api_user(token: String, db: &DbConn) -> Option<ApiProfile> {
    let me: DiscordUser = oauth_request("users/@me", token.clone())
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let data: Vec<UserJoined> = users::table
        .inner_join(
            connections::table.on(users::discord_id
                .eq(connections::discord_id)
                .and(users::discord_id.eq(me.id.parse::<i64>().unwrap()))),
        )
        .select((users::discord_id, users::contributions, connections::github))
        .load(&db.0)
        .unwrap();

    if data.is_empty() {
        let mut github: String = "".to_string();
        let connections: Vec<ApiUserConnection> =
            oauth_request("users/@me/connections", token.clone())
                .await
                .unwrap()
                .json()
                .await
                .unwrap();

        for conn in connections {
            if conn.conn_type.to_lowercase() == "github" {
                github = conn.name;
                break;
            }
        }

        if github == "" {
            return None;
        }

        let contribs = get_contributions(github.clone());

        make_new_user(
            UserJoined {
                discord_id: me.id.parse().unwrap(),
                contributions: contribs,
                github: github.clone(),
            },
            &db,
        );
        add_user_guilds(token.clone(), &db, me.id.parse().unwrap()).await;

        Some(ApiProfile {
            tag: format!("{}#{}", me.username, me.discriminator),
            github: github.clone(),
            avatar_url: format!(
                "https://cdn.discordapp.com/avatars/{}/{}.png",
                me.id, me.avatar
            ),
            contributions: contribs,
        })
    } else if let Some(db_user) = data.get(0) {
        Some(ApiProfile {
            tag: format!("{}#{}", me.username, me.discriminator),
            github: db_user.github.clone(),
            avatar_url: format!(
                "https://cdn.discordapp.com/avatars/{}/{}.png",
                me.id, me.avatar
            ),
            contributions: db_user.contributions.clone(),
        })
    } else {
        None
    }
}

pub fn get_contributions(username: String) -> i32 {
    let activity: ApiActivity =
        blocking::get(format!("https://github-contributions.now.sh/api/v1/{}", username).as_str())
            .unwrap()
            .json()
            .unwrap();
    activity.years.last().unwrap().total
}

pub fn make_new_user(user: UserJoined, db: &PgConnection) {
    let mut time = Utc::now() + Duration::minutes(5);
    let new_user = NewUser {
        discord_id: user.discord_id,
        contributions: user.contributions,
        expires: time.naive_utc(),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(db)
        .expect("Unable to insert");

    time = Utc::now() + Duration::days(1);
    let new_connection = NewConnection {
        discord_id: user.discord_id,
        github: user.github,
        expires: time.naive_utc(),
    };

    diesel::insert_into(connections::table)
        .values(&new_connection)
        .execute(db)
        .expect("Unable to insert");
}

pub async fn add_user_guilds(token: String, db: &DbConn, user_id: i64) {
    let discord_guilds: Vec<DiscordGuild> = oauth_request("users/@me/guilds", token)
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    for guild in discord_guilds {
        let new_guild = NewGuild {
            discord_id: user_id,
            guild_id: guild.id.parse().unwrap(),
        };
        diesel::insert_into(guilds::table)
            .values(&new_guild)
            .on_conflict_do_nothing()
            .execute(&db.0)
            .expect("Unable to insert");
    }
}

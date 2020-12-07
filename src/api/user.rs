use crate::api::models::{ApiUserConnection, DiscordUser};
use crate::api::{json, ApiResponse};
use crate::db::guard::DbConn;
use crate::models::{ApiActivity, ApiProfile};
use crate::oauth::oauth_request;
use chrono::{Duration, Utc};
use diesel::{RunQueryDsl, PgConnection, QueryDsl, ExpressionMethods, JoinOnDsl, BoolExpressionMethods};
use reqwest::blocking;
use rocket::get;
use rocket::http::{Cookies, Status};
use crate::schemas::{NewUser, NewConnection};
use crate::schemas::diesel::users;
use crate::schemas::diesel::connections;

#[derive(Debug, Queryable)]
pub struct UserJoined {
    pub discord_id: i64,
    pub contributions: i32,
    pub github: String
}

#[get("/user")]
pub fn get_user(cookies: Cookies, db: DbConn) -> ApiResponse {
    let token = cookies.get("discord_token");
    return match token {
        Some(token) => match get_api_user(token.value().to_string(), &db) {
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

pub fn get_api_user(token: String, db: &PgConnection) -> Option<ApiProfile> {
    let me: DiscordUser = oauth_request("users/@me", token.clone())
        .unwrap()
        .json()
        .unwrap();

    let data: Vec<UserJoined> = users::table
        .inner_join(connections::table.on(users::discord_id.eq(connections::discord_id)
            .and(users::discord_id.eq(me.id.parse::<i64>().unwrap()))))
        .select((users::discord_id, users::contributions, connections::github))
        .load(db)
        .unwrap();

    if data.is_empty() {

        let mut github: String = "".to_string();
        let connections: Vec<ApiUserConnection> = oauth_request("users/@me/connections", token.clone())
            .unwrap()
            .json()
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

        make_new_user(UserJoined {
            discord_id: me.id.parse().unwrap(),
            contributions: contribs,
            github: github.clone(),
        }, &db);

        Some(ApiProfile {
            tag: format!("{}#{}", me.username, me.discriminator),
            github: github.clone(),
            avatar_url: format!(
                "https://cdn.discordapp.com/avatars/{}/{}.png",
                me.id, me.avatar
            ),
            contributions: contribs
        })
    } else if let Some(db_user) = data.get(0)  {
        Some(ApiProfile {
            tag: format!("{}#{}", me.username, me.discriminator),
            github: db_user.github.clone(),
            avatar_url: format!(
                "https://cdn.discordapp.com/avatars/{}/{}.png",
                me.id, me.avatar
            ),
            contributions: db_user.contributions.clone()
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


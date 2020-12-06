use crate::api::models::{ApiUser, ApiUserConnection};
use crate::api::{json, ApiResponse};
use crate::models::{Activity, User};
use crate::oauth::oauth_request;
use reqwest::blocking;
use rocket::get;
use rocket::http::{Cookies, Status};
use rocket_contrib::json::JsonValue;
use serde::Serialize;
use std::fmt;

#[get("/user")]
pub fn get_user(cookies: Cookies) -> ApiResponse {
    let token = cookies.get("discord_token");
    return match token {
        Some(token) => match get_api_user(token.value().to_string()) {
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

#[get("/user/contributions?<username>")]
pub fn get_user_contributions(username: String) -> ApiResponse {
    let res =
        blocking::get(format!("https://github-contributions.now.sh/api/v1/{}", username).as_str());
    return match res {
        Ok(res) => ApiResponse {
            json: res.json().unwrap(),
            status: Status::Ok,
        },
        Err(_) => ApiResponse {
            json: json!({"Error": "Bad request"}),
            status: Status::BadRequest,
        },
    };
}

pub fn get_api_user(token: String) -> Option<User> {
    let me: ApiUser = oauth_request("users/@me", token.clone())
        .unwrap()
        .json()
        .unwrap();

    let mut github: String = "".to_string();
    let connections: Vec<ApiUserConnection> = oauth_request("users/@me/connections", token.clone())
        .unwrap()
        .json()
        .unwrap();

    for (conn) in connections {
        if conn.conn_type.to_lowercase() == "github" {
            github = conn.name;
            break;
        }
    }

    if github == "" {
        return None;
    }

    let activity: Activity =
        blocking::get(format!("https://github-contributions.now.sh/api/v1/{}", github).as_str())
            .unwrap()
            .json()
            .unwrap();

    Some(User {
        name: me.username,
        github,
        id: me.id.parse::<u64>().unwrap(),
        activity,
        avatar_url: format!(
            "https://cdn.discordapp.com/avatars/{}/{}.png",
            me.id, me.avatar
        ),
    })
}

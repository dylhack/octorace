use crate::api::models::{ApiUserConnection, DiscordUser};
use crate::api::user::{get_contributions, make_new_user, UserJoined};
use crate::config::Config;
use crate::db::guard::DbConn;
use crate::db::pool::Pool;
use crate::oauth::{oauth_request, OauthClient};
use oauth2::reqwest::async_http_client;
use oauth2::{AuthorizationCode, CsrfToken, Scope, TokenResponse};
use rocket::get;
use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;
use rocket::State;
use time::Duration;

#[get("/")]
pub fn oauth_main(client: State<OauthClient>) -> Redirect {
    let (authorize_url, _csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .add_scope(Scope::new("connections".to_string()))
        .add_scope(Scope::new("guilds".to_string()))
        .url();

    Redirect::to(authorize_url.to_string())
}

#[allow(unused_variables)]
#[get("/callback?<code>&<state>")]
pub async fn oauth_callback(
    client: State<'_, OauthClient>,
    code: String,
    state: String,
    jar: &CookieJar<'_>,
    db: DbConn<'_>,
    config: State<'_, Config>,
) -> Option<Redirect> {
    let code = AuthorizationCode::new(code);
    let token_res = client
        .exchange_code(code)
        .request_async(async_http_client)
        .await;

    return if let Ok(token) = token_res {
        let discord_token = token.access_token();

        let cookie = Cookie::build("discord_token", discord_token.secret().clone())
            .path("/")
            .http_only(false)
            .max_age(Duration::hours(12))
            .finish();

        jar.add_private(cookie);

        let me: DiscordUser =
            match oauth_request("users/@me", token.access_token().secret().clone())
                .await
                .unwrap()
                .json()
                .await
            {
                Ok(me) => me,
                Err(_) => return Some(Redirect::to("/error")),
            };

        let user_id_cookie = Cookie::build("discord_id", me.id.clone())
            .path("/")
            .http_only(false)
            .finish();

        jar.add_private(user_id_cookie);

        create_user(discord_token.secret().clone(), &db, &config, me).await;

        Some(Redirect::to("/"))
    } else {
        Some(Redirect::to("/error"))
    };
}

async fn create_user(token: String, pool: &Pool, config: &Config, me: DiscordUser) {
    let exists = sqlx::query!(
        "SELECT * FROM octorace.connections WHERE discord_id = $1",
        me.id.parse::<i64>().unwrap()
    )
    .fetch_optional(pool)
    .await
    .unwrap();

    let mut github: String = "".to_string();
    let contribs;

    if let Some(db_user) = exists {
        contribs = get_contributions(db_user.github, config).await;
        sqlx::query!(
            "UPDATE octorace.users SET contributions = $1 WHERE discord_id = $2",
            contribs,
            db_user.discord_id
        )
        .execute(pool)
        .await
        .expect("Unable to update");
    } else {
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

        if github.is_empty() {
            return;
        }
        contribs = get_contributions(github.clone(), config).await;

        make_new_user(
            UserJoined {
                discord_id: me.id.parse().unwrap(),
                contributions: contribs,
                github: github.clone(),
                tag: "".to_string(),
                avatar_url: "".to_string(),
            },
            &me,
            pool,
        )
        .await;
    }
}

use crate::oauth::{OauthClient, oauth_request};
use oauth2::reqwest::async_http_client;
use oauth2::{AuthorizationCode, CsrfToken, Scope, TokenResponse};
use rocket::get;
use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;
use rocket::State;
use crate::api::user::{get_contributions, make_new_user, UserJoined, add_user_guilds};
use crate::models::ApiProfile;
use crate::db::guard::DbConn;
use crate::api::models::{ApiUserConnection, DiscordUser};
use crate::db::pool::Pool;

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
    db: DbConn<'_>
) -> Redirect {
    let code = AuthorizationCode::new(code);
    let token_res = client
        .exchange_code(code)
        .request_async(async_http_client)
        .await;

    return if let Ok(token) = token_res {

        let discord_token = token.access_token();

        let cookie = Cookie::build("discord_token", discord_token.secret().clone())
            .path("/")
            .finish();

        jar.add(cookie);

        create_user(discord_token.secret().clone(), &db).await;

        Redirect::to("/")
    } else {
        "Something went wrong..".to_string();
        Redirect::to("")
    };
}

async fn create_user(token: String, pool: &Pool) {

    let me: DiscordUser = oauth_request("users/@me", token.clone())
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let exists = sqlx::query!("SELECT * FROM octorace.users WHERE discord_id = $1", me.id.parse::<i64>().unwrap())
        .fetch_optional(pool)
        .await
        .unwrap();

    if let Some(_) = exists {
        return;
    }

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

    if github.is_empty() {
        return;
    }

    let contribs = get_contributions(github.clone()).await;

    make_new_user(
        UserJoined {
            discord_id: me.id.parse().unwrap(),
            contributions: contribs,
            github: github.clone(),
        },
        &me,
        pool,
    )
        .await;
    add_user_guilds(token.clone(), pool, me.id.parse().unwrap()).await;
}

use crate::oauth::OauthClient;
use oauth2::reqwest::async_http_client;
use oauth2::{AuthorizationCode, CsrfToken, Scope, TokenResponse};
use rocket::get;
use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;
use rocket::State;

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
            .secure(true)
            .finish();

        jar.add(cookie);

        Redirect::to("/")
    } else {
        "Something went wrong..".to_string();
        Redirect::to("")
    };
}

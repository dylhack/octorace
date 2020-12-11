pub mod routes;

use crate::config::Config;
use oauth2::basic::{BasicClient, BasicErrorResponseType, BasicTokenType};
use oauth2::{
    AuthUrl, ClientId, ClientSecret, EmptyExtraTokenFields, RedirectUrl, StandardErrorResponse,
    StandardTokenResponse, TokenUrl,
};
use reqwest::Client;

type OauthClient = oauth2::Client<
    StandardErrorResponse<BasicErrorResponseType>,
    StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    BasicTokenType,
>;

pub fn create_oauth_client(config: Config) -> OauthClient {
    let discord_client_id = ClientId::new(config.client_id.to_string());
    let discord_client_secret = ClientSecret::new(config.client_secret);

    let auth_url = AuthUrl::new("https://discord.com/api/oauth2/authorize".to_string())
        .expect("Invalid authorization endpoint URL");

    let token_url = TokenUrl::new("https://discord.com/api/oauth2/token".to_string())
        .expect("Invalid token endpoint URL");

    BasicClient::new(
        discord_client_id,
        Some(discord_client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_url(
        RedirectUrl::new(format!("{}/oauth/callback", config.domain))
            .expect("Invalid redirect URL"),
    )
}

pub async fn oauth_request(url: &str, token: String) -> Option<reqwest::Response> {
    //  -> Result<Response, Box<dyn std::error::Error>>
    let client = Client::new();
    match client
        .get(format!("https://discordapp.com/api/{}", url).as_str())
        .bearer_auth(token)
        .send()
        .await
    {
        Ok(res) => Some(res),
        Err(_) => None,
    }
}

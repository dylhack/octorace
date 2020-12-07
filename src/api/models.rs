use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiUserConnection {
    #[serde(alias = "type")]
    pub(crate) conn_type: String,
    pub(crate) name: String,
}

#[derive(Debug, Deserialize)]
pub struct DiscordUser {
    pub(crate) id: String,
    pub(crate) username: String,
    pub(crate) avatar: String,
    pub(crate) discriminator: String,
    pub(crate) email: String,
}

#[derive(Debug, Deserialize)]
pub struct DiscordGuild {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) icon: Option<String>,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiGuild {
    pub name: String,
    pub id: String,
    pub icon_url: String,
    pub profiles: Vec<ApiProfile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiProfile {
    pub tag: String,
    pub contributions: i32,
    pub avatar_url: String,
    pub github: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiActivity {
    pub years: Vec<ApiYear>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiYear {
    pub total: i32,
}

use serde::{Deserialize, Serialize};
use diesel::sql_types::{Text, Integer};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiGuild {
    pub name: String,
    pub id: String,
    pub icon_url: String,
    pub profiles: Vec<ApiProfile>,
}

#[derive(Debug, Serialize, Deserialize, QueryableByName)]
pub struct ApiProfile {
    #[sql_type = "Text"]
    pub tag: String,
    #[sql_type = "Integer"]
    pub contributions: i32,
    #[sql_type = "Text"]
    pub avatar_url: String,
    #[sql_type = "Text"]
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

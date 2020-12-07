use chrono::{NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiGuild {
    pub name: String,
    pub github_name: String,
    pub id: u64,
    pub users: Vec<ApiUser>,
    pub icon_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiUser {
    pub name: String,
    pub github: String,
    pub id: u64,
    pub activity: ApiActivity,
    pub avatar_url: String,
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
    pub contributions: Vec<ApiContribution>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiContribution {
    pub date: NaiveDate,
    pub count: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiYear {
    pub year: String,
    pub total: i32,
    pub range: ApiRange,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiRange {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

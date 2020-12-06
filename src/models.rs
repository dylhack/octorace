use chrono::{Date, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Guild {
    pub name: String,
    pub github_name: String,
    pub id: u64,
    pub users: Vec<User>,
    pub icon_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub github: String,
    pub id: u64,
    pub activity: Activity,
    pub avatar_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Activity {
    pub years: Vec<Year>,
    pub contributions: Vec<Contribution>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contribution {
    pub date: NaiveDate,
    pub count: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Year {
    pub year: String,
    pub total: u64,
    pub range: Range,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Range {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

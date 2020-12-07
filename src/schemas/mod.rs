use self::diesel::*;
use chrono::NaiveDateTime;

pub mod diesel;

#[derive(Queryable, Debug)]
pub struct User {
    pub discord_id: i64,
    pub contributions: i32,
    pub expires: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub discord_id: i64,
    pub contributions: i32,
    pub expires: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct Connection {
    pub discord_id: i64,
    pub github: String,
    pub expires: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "connections"]
pub struct NewConnection {
    pub discord_id: i64,
    pub github: String,
    pub expires: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct Guild {
    pub discord_id: i64,
    pub guild_id: i64,
}

#[derive(Insertable)]
#[table_name = "guilds"]
pub struct NewGuild {
    pub discord_id: i64,
    pub guild_id: i64,
}

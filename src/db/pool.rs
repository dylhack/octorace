use std::env;

use sqlx::postgres::PgPoolOptions;
use sqlx::Postgres;

pub type Pool = sqlx::Pool<Postgres>;

pub async fn pool() -> Pool {
    PgPoolOptions::new()
        .connect(database_url().as_str())
        .await
        .expect("Unable to create db pool")
}

#[allow(clippy::or_fun_call)]
fn database_url() -> String {
    env::var("DATABASE_URL").unwrap_or(
        "postgres://postgres:password123@localhost/postgres"
            .to_string(),
    )
}

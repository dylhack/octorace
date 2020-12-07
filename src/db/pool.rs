use std::env;

use diesel::r2d2::ConnectionManager;
use diesel::{r2d2, Connection, PgConnection};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn pool() -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(database_url());
    Pool::new(manager).expect("Unable to create db pool: ")
}

#[allow(clippy::or_fun_call)]
fn database_url() -> String {
    env::var("DATABASE_URL").unwrap_or(
        "postgres://postgres:password123@localhost/postgres?options=-c search_path%3Doctorace"
            .to_string(),
    )
}

pub fn pg_connection() -> PgConnection {
    PgConnection::establish(database_url().as_str()).expect("Unable to connect: ")
}

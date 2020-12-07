use std::ops::Deref;

use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::{request, Request, State};

use crate::db::pool::Pool;
use rocket::outcome::Outcome;

pub struct DbConn(pub PooledConnection<ConnectionManager<PgConnection>>);

unsafe impl Send for DbConn {}

unsafe impl Sync for DbConn {}


#[rocket::async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    async fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, Self::Error> {
        let pool = request.guard::<State<Pool>>().await.unwrap();
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

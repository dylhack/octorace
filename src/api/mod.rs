pub mod guilds;
pub(crate) mod models;
pub mod user;

use rocket::http::ContentType;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response;
use rocket::response::Responder;
use rocket::response::Response;
use rocket_contrib::json;
use rocket_contrib::json::JsonValue;

#[derive(Debug)]
pub struct ApiResponse {
    json: JsonValue,
    status: Status,
}

impl<'r> Responder<'r, 'static> for ApiResponse {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        Response::build_from(self.json.respond_to(req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

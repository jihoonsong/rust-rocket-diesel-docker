use rocket::http::Status;
use rocket::serde::json::{json, Value as JsonValue};
use rocket::Request;

#[catch(default)]
pub fn default_error(status: Status, request: &Request) -> JsonValue {
    json!({
        "status": status.code,
        "uri": request.uri(),
        "timestamp(UTC)": webserver::get_readable_timestamp(),
    })
}

#[catch(404)]
pub fn not_found_error(request: &Request) -> JsonValue {
    json!({
        "status": 404,
        "error": "Not Found",
        "uri": request.uri(),
        "timestamp(UTC)": webserver::get_readable_timestamp(),
    })
}

#[catch(500)]
pub fn internal_server_error() -> JsonValue {
    json!({
        "status": 500,
        "error": "Internal Server Error",
        "timestamp(UTC)": webserver::get_readable_timestamp(),
    })
}

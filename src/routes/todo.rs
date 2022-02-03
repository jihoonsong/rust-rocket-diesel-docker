use crate::db;

use rocket::serde::json::Value as JsonValue;

#[get("/")]
pub async fn get_all(db: db::Db) -> Option<JsonValue> {
    None
}

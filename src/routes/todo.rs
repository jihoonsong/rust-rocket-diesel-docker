use crate::db;

use rocket::figment::value::Value as FigmentValue;
use rocket::serde::json::{json, Value as JsonValue};

#[get("/")]
pub async fn get_all(db: db::Db) -> Option<JsonValue> {
    match db::todo::get_all(&db).await {
        Ok(todos) => Some(json!({
            "status": 200,
            "result": FigmentValue::serialize(todos).unwrap(),
        })),
        Err(_) => None,
    }
}

#[get("/<id>")]
pub async fn get(db: db::Db, id: i32) -> Option<JsonValue> {
    None
}

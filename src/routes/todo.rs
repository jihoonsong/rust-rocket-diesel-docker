use crate::db;
use crate::models::todo::NewTodo;

use rocket::figment::value::Value as FigmentValue;
use rocket::serde::json::{json, Json, Value as JsonValue};

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

#[get("/?<order>")]
pub async fn get_all_order_by(db: db::Db, order: &str) -> Option<JsonValue> {
    match order.to_ascii_lowercase().as_str() {
        "asc" => get_all_order_by_asc(&db).await,
        "desc" => get_all_order_by_desc(&db).await,
        _ => None,
    }
}

#[get("/<id>")]
pub async fn get(db: db::Db, id: i32) -> Option<JsonValue> {
    match db::todo::get(&db, id).await {
        Ok(todo) => Some(json!({
            "status": 200,
            "result": todo.to_json(),
        })),
        Err(_) => None,
    }
}

#[post("/", format = "json", data = "<new_todo>")]
pub async fn create(db: db::Db, new_todo: Json<NewTodo>) -> Option<JsonValue> {
    match db::todo::create(&db, new_todo.into_inner()).await {
        Ok(todo) => Some(json!({
            "status": 200,
            "result": todo.to_json(),
        })),
        Err(_) => None,
    }
}

#[put("/<id>", format = "json", data = "<new_todo>")]
pub async fn update(db: db::Db, id: i32, new_todo: Json<NewTodo>) -> Option<JsonValue> {
    match db::todo::update(&db, id, new_todo.into_inner()).await {
        Ok(todo) => Some(json!({
            "status": 200,
            "result": todo.to_json(),
        })),
        Err(_) => None,
    }
}

#[delete("/<id>")]
pub async fn delete(db: db::Db, id: i32) -> Option<JsonValue> {
    match db::todo::delete(&db, id).await {
        Ok(todo) => Some(json!({
            "status": 200,
            "result": todo.to_json(),
        })),
        Err(_) => None,
    }
}

async fn get_all_order_by_asc(db: &db::Db) -> Option<JsonValue> {
    match db::todo::get_all_order_by_asc(db).await {
        Ok(todos) => Some(json!({
            "status": 200,
            "result": FigmentValue::serialize(todos).unwrap(),
        })),
        Err(_) => None,
    }
}

async fn get_all_order_by_desc(db: &db::Db) -> Option<JsonValue> {
    match db::todo::get_all_order_by_desc(db).await {
        Ok(todos) => Some(json!({
            "status": 200,
            "result": FigmentValue::serialize(todos).unwrap(),
        })),
        Err(_) => None,
    }
}

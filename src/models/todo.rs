use chrono::{DateTime, Utc};
use rocket::serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Todo {
    pub id: i32,
    pub creator_name: String,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct TodoJson {
    pub id: i32,
    pub creator_name: String,
    pub title: String,
    pub description: String,
    pub created_at: String,
}

impl Todo {
    pub fn to_json(self) -> TodoJson {
        TodoJson {
            id: self.id,
            creator_name: self.creator_name,
            title: self.title,
            description: self.description,
            created_at: self.created_at.format(webserver::DATE_FORMAT).to_string(),
        }
    }
}

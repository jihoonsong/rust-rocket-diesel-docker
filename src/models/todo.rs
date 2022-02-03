use chrono::{DateTime, Utc};

#[derive(Queryable)]
pub struct Todo {
    pub id: i32,
    pub creator_name: String,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
}

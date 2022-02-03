use crate::db;
use crate::models::todo::Todo;
use crate::schema::todo::todos::dsl::todos as all_todos;

use diesel::{self, prelude::*, result::QueryResult};

pub async fn get_all(db: &db::Db) -> QueryResult<Vec<Todo>> {
    db.run(|db| all_todos.load::<Todo>(db)).await
}

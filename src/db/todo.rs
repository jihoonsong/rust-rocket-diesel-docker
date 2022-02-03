use crate::db;
use crate::models::todo::{NewTodo, Todo};
use crate::schema::todo::todos::{self, dsl::todos as all_todos};

use diesel::{self, prelude::*, result::QueryResult};

pub async fn get_all(db: &db::Db) -> QueryResult<Vec<Todo>> {
    db.run(|db| all_todos.load::<Todo>(db)).await
}

pub async fn get(db: &db::Db, id: i32) -> QueryResult<Todo> {
    db.run(move |db| all_todos.find(id).get_result(db)).await
}

pub async fn create(db: &db::Db, new_todo: NewTodo) -> QueryResult<Todo> {
    db.run(move |db| {
        diesel::insert_into(todos::table)
            .values(&new_todo)
            .get_result(db)
    })
    .await
}

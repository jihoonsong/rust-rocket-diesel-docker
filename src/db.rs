pub mod todo;

use rocket::fairing::AdHoc;
use rocket_sync_db_pools::{database, diesel};

#[database("postgres")]
pub struct Db(diesel::PgConnection);

pub fn init() -> AdHoc {
    AdHoc::on_ignite("Ignite Diesel and PostgreSQL", |rocket| async {
        rocket.attach(Db::fairing())
    })
}

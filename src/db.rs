pub mod todo;

use rocket::fairing::AdHoc;
use rocket::{Build, Rocket};
use rocket_sync_db_pools::{database, diesel};

#[database("postgres")]
pub struct Db(diesel::PgConnection);

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    embed_migrations!("migrations");

    let db = Db::get_one(&rocket)
        .await
        .expect("Database connection failed");
    db.run(|db| embedded_migrations::run(db))
        .await
        .expect("Diesel migrations failed");

    rocket
}

pub fn init() -> AdHoc {
    AdHoc::on_ignite("Ignite Diesel and PostgreSQL", |rocket| async {
        rocket
            .attach(Db::fairing())
            .attach(AdHoc::on_ignite("Run Diesel migrations", run_migrations))
    })
}

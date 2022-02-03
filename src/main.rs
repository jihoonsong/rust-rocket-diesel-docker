#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod db;
mod errors;
mod models;
mod routes;
mod schema;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(db::init())
        .mount("/", routes![routes::hello_world])
        .register(
            "/",
            catchers![
                errors::default_error,
                errors::not_found_error,
                errors::internal_server_error,
            ],
        )
}
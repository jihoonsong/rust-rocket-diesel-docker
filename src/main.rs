#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod db;
mod errors;
mod models;
mod routes;
mod schema;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(db::init())
        .mount(
            "/todos",
            routes![
                routes::todo::get_all,
                routes::todo::get_all_order_by,
                routes::todo::get,
                routes::todo::create,
                routes::todo::update,
                routes::todo::delete,
            ],
        )
        .register(
            "/",
            catchers![
                errors::default_error,
                errors::not_found_error,
                errors::internal_server_error,
            ],
        )
}

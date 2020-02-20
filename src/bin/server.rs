#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use test_rocket_app::controllers::users;
use test_rocket_app::init_pool;

#[catch(404)]
fn not_found() -> String {
    "{\"status\": \"Not Found\", \"Message\": \"Page not found\"}".to_string()
}

fn main() {
    dotenv().ok();
    rocket::ignite()
        .manage(init_pool())
        .mount("/api", routes![users::index, users::get, users::store])
        .register(catchers![not_found])
        .launch();
}

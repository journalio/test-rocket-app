extern crate diesel;

use dotenv::dotenv;
use test_rocket_app::models::NewUser;
use test_rocket_app::repositories::users;
use test_rocket_app::{init_pool, DbConn};

fn main() {
    dotenv().ok();
    let pool = init_pool();
    let conn = DbConn(pool.get().unwrap());

    
    let new_user = NewUser {
        full_name: "Kasper van den berg".into(),
        email: "s1101481@student.hsleiden.nl".into(),
        password_hash: "blabblablabcrypt nog nodig".into(),
    };

    let created_user = users::insert(new_user, &conn).expect("Failed to create user");

    println!("{}", created_user.full_name);
    println!("----------\n");
    println!("{}", created_user.email);
}

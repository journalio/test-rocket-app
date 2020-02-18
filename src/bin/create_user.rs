extern crate diesel;

use self::diesel::prelude::*;
use test_rocket_app::establish_connection;
use test_rocket_app::models::User;

fn main() {
    use test_rocket_app::schema::users::dsl::*;

    let connection = establish_connection();
    let new_user = User {
        id: Default::default(),
        full_name: "Wesley Klop".to_string(),
        email: "wesley19097@gmail.com".to_string(),
        password_hash: "blabblablabcrypt nog nodig".to_string(),
    };
    let results = diesel::insert_into(users)
        .values(new_user)
        .get_results(&connection);

    println!("Inserted {} users", results.len());
    for user in results {
        println!("{}", user.full_name);
        println!("----------\n");
        println!("{}", user.email);
    }
}

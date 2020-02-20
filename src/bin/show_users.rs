use dotenv::dotenv;
use test_rocket_app::{init_pool, repositories::users, DbConn};

fn main() {
    dotenv().ok();
    let pool = init_pool();
    let conn = DbConn(pool.get().unwrap());

    let results = users::all(&conn).expect("Failed to get users");
    println!("Displaying {} users:", results.len());
    for user in results {
        println!("- {}", user.full_name);
        println!("  {}", user.email);
        println!();
    }
}

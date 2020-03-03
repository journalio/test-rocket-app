use diesel::pg::PgConnection;
use rocket_contrib::database;

/// Use this type as database connection.
/// This type can be changed to any
/// other database driver.
pub type DbConn = Postgres;

/// Describes the connectiontype being used
/// by the connection.
pub trait ConnectionType {
    type Connection;
}

/// The postgres database
#[database("postgres")]
pub struct Postgres(PgConnection);

impl ConnectionType for Postgres {
    type Connection = PgConnection;
}

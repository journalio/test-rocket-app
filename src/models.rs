use diesel::sql_types::Timestamp;
use uuid::Uuid;

use super::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: Uuid,
    pub full_name: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub full_name: &'a str,
    pub email: &'a str,
    pub password_hash: &'a str,
}

#[derive(Queryable)]
pub struct Tag {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub color: u8,
    pub created_at: Timestamp,
    pub created_by: Uuid,
    pub updated_at: Timestamp,
}

#[derive(Queryable)]
pub struct Item {
    pub id: Uuid,
    pub title: String,
    pub item_type: String,
    pub created_at: Timestamp,
    pub created_by: Uuid,
    pub updated_at: Timestamp,
    pub owned_by: Uuid,
}

#[derive(Queryable)]
pub struct Todo {
    pub id: Uuid,
    pub content: String,
    pub done: bool,
}

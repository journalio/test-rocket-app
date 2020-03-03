#![allow(clippy::single_component_path_imports)]

use diesel::sql_types::Timestamp;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::schema::users;

#[derive(Queryable, Serialize, AsChangeset, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub full_name: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub full_name: String,
    pub email: String,
    pub password_hash: String,
}

impl NewUser {
    pub fn hash_password(self) -> Self {
        let password_hash = bcrypt::hash(&self.password_hash, bcrypt::DEFAULT_COST).unwrap();

        Self {
            password_hash,
            ..self
        }
    }
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

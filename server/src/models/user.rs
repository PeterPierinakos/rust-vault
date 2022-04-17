use crate::schema::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct UserForm {
    pub username: String,
    pub password: String,
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

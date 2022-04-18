use crate::schema::*;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct TextForm {
    pub content: String,
}

#[derive(Queryable)]
pub struct Text {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name="texts"]
pub struct NewText {
    pub content: String,
}

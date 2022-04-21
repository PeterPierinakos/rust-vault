use crate::schema::*;
use serde::{Serialize, Deserialize};


#[derive(Deserialize, Serialize)]
pub struct TextForm {
    pub text_name: String,
    pub content: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SmallTextForm {
    pub text_name: String,
}

#[derive(Queryable, Debug)]
pub struct Text {
    pub id: i32,
    pub text_name: String,
    pub owner: i32,
    pub content: String,
}

#[derive(Insertable)]
#[table_name="texts"]
pub struct NewText {
    pub text_name: String,
    pub owner: i32,
    pub content: String,
}

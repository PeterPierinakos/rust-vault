use crate::schema::*;
use serde::{Serialize, Deserialize};
use diesel::pg::data_types::PgNumeric;

#[derive(Deserialize, Serialize)]
pub struct TextForm {
    pub content: String,
}

#[derive(Queryable)]
pub struct Text {
    pub id: i32,
    pub owner: PgNumeric,
    pub content: String,
}

#[derive(Insertable)]
#[table_name="texts"]
pub struct NewText {
    pub owner: PgNumeric,
    pub content: String,
}

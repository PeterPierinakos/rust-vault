use actix_web::{Responder, HttpResponse, web};

use http::StatusCode;
use actix_session::Session;
use crate::utils::verify::verify_session;
use crate::schema::*;
use crate::models::text::*;


use diesel::prelude::*;

use crate::structs::app_state::AppState;
use crate::utils::db::pg_pool_handler;

pub async fn upload_text(state: web::Data<AppState>, session: Session, req: web::Form<TextForm>) -> impl Responder {
    let pool = &state.pool;
    let connection = pg_pool_handler(&pool).unwrap();

    let res = verify_session(&session);
    if res == false {
        return HttpResponse::build(StatusCode::UNAUTHORIZED)
            .body("Not authenticated");
    }

    let id = session.get::<i32>("id").unwrap().unwrap();

    diesel::insert_into(texts::table)
        .values(NewText {
            text_name: req.text_name.clone(),
            owner: id,
            content: req.content.clone(),
        })
        .get_result::<Text>(&connection)
        .unwrap();

    HttpResponse::build(StatusCode::OK)
        .body("Success")
}

use actix_web::{Responder, HttpResponse, web};
use diesel::insert_into;
use http::StatusCode;
use actix_session::Session;
use crate::utils::session_utils::verify_session;
use crate::schema::*;
use crate::models::text::*;
use crate::models::user::*;
use diesel::pg::data_types::PgNumeric;
use diesel::prelude::*;
use diesel::r2d2::*;
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
    let id_s = id.to_string();

    diesel::insert_into(texts::table)
        .values(NewText {
            owner: PgNumeric::Positive {
                weight: id_s.len() as i16, 
                scale: id_s.len() as u16,
                digits: vec![id as i16],
            },
            content: req.content.clone(),
        })
        .get_result::<Text>(&connection)
        .unwrap();

    HttpResponse::build(StatusCode::OK)
        .body("Success")
}

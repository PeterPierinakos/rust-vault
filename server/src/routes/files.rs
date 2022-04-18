use actix_web::{Responder, HttpResponse, web};
use http::StatusCode;
use actix_session::Session;
use crate::models::text::TextForm;
use crate::utils::session_utils::verify_session;

pub async fn upload_text(session: Session, req: web::Form<TextForm>) -> impl Responder {
    if !verify_session(&session) {
        return HttpResponse::build(StatusCode::UNAUTHORIZED)
            .body("Not authenticated");
    }         
    HttpResponse::build(StatusCode::OK)
        .body("Success")
}

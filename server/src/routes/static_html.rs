use actix_web::{Responder, HttpResponse};
use http::StatusCode;

pub async fn index_html() -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../../web/static/index.html"))
}

pub async fn login_html() -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../../web/static/login.html"))
}


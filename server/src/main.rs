use actix_web::{HttpServer, App, web, Responder};
use std::io::Error;
use server::config::*;
use log::info;

async fn index() -> Result<impl Responder, Error> {
    Ok("Hello!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("Server adress: {:?}", ADRESS);
    info!("Starting HTTP server on port {}", PORT);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
        .bind(format!("{}:{}", format!("{}.{}.{}.{}", 
                                        ADRESS[0],
                                        ADRESS[1],
                                        ADRESS[2],
                                        ADRESS[3],
                                       ), PORT))?
        .run()
        .await
}

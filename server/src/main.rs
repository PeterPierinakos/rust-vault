use actix_web::{HttpServer, HttpResponse, App, web};
use diesel::r2d2::State;
use server::config::*;
use log::info;
use server::utils::db::{establish_connection};
use server::structs::app_state::AppState;
use server::routes::authentication::*;
use server::routes::static_html::*;
use server::routes::secret::*;
use actix_session::{SessionMiddleware, storage::RedisActorSessionStore};
use actix_web::cookie::Key;
use http::StatusCode;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    info!("Server adress: {:?}", ADRESS);
    info!("Starting HTTP server on port {}", PORT);
    
    let secret_redis_key = Key::generate();
    
    HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::new(
                    RedisActorSessionStore
                        ::new(
                            format!("{}:{}", 
                                format!("{}.{}.{}.{}", 
                                    ADRESS[0],
                                    ADRESS[1],
                                    ADRESS[2],
                                    ADRESS[3],
                            ), PORT)),
                    secret_redis_key.clone()
                )
            )
            .app_data(web::Data::new(AppState {
                pool: establish_connection(),
            }))
            .route("/", web::get().to(index_html))
            .route("/login", web::get().to(login_html))
            .route("/api/signup", web::post().to(signup))
            .route("/api/login", web::get().to(login))
            .route("/api/delete", web::delete().to(delete_account))
    })
        .bind(format!("{}:{}", format!("{}.{}.{}.{}", 
                                        ADRESS[0],
                                        ADRESS[1],
                                        ADRESS[2],
                                        ADRESS[3]
                                       ), PORT))?
        .run()
        .await
}

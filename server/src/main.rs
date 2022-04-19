use actix_web::{HttpServer, App, web};
use core::time::Duration;
use server::config::*;
use log::info;
use server::utils::db::establish_connection;
use server::structs::app_state::AppState;
use server::routes::authentication::*;
use server::routes::static_html::*;
use server::routes::secret::*;
use server::routes::files::*;
use actix_session::{SessionMiddleware, storage::RedisActorSessionStore};
use actix_web::cookie::Key;
use redis::aio::ConnectionManager;
use actix_extensible_rate_limit::backend::redis::RedisBackend;
use actix_extensible_rate_limit::RateLimiter;
use actix_extensible_rate_limit::backend::SimpleInputFunctionBuilder;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    info!("Server adress: {:?}", ADRESS);
    info!("Starting HTTP server on port {}", PORT);
    
    let secret_redis_key = Key::generate();

    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let manager = ConnectionManager::new(client).await.unwrap();
    let backend = RedisBackend::builder(manager).build();        

    HttpServer::new(move || {
        let input = SimpleInputFunctionBuilder::new(Duration::from_secs(60), RATELIMIT_REQUESTS_ALLOWED_PER_60_SECONDS)
            .real_ip_key()
            .build();

        let rate_middleware = RateLimiter::builder(backend.clone(), input).add_headers().build();

        App::new()
            .app_data(web::Data::new(AppState {
                pool: establish_connection(),
            }))
            .wrap(rate_middleware)
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
                            ), REDIS_PORT)),
                    secret_redis_key.clone()
                )
            )
            .route("/", web::get().to(index_html))
            .route("/login", web::get().to(login_html))
            .route("/signup", web::get().to(signup_html))
            .route("/upload", web::get().to(upload_html))
            .route("/home", web::get().to(home_html))
            .route("/api/signup", web::post().to(signup))
            .route("/api/login", web::get().to(login))
            .route("/api/delete", web::delete().to(delete_account))
            .route("/api/upload", web::post().to(upload_text))
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

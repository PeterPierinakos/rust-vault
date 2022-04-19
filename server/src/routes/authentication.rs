use actix_web::{Responder, web, HttpResponse};
use crate::schema::*;
use crate::structs::app_state::AppState;
use crate::models::user::*;
use diesel::prelude::*;
use crate::utils::db::pg_pool_handler;
use http::StatusCode;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use actix_session::Session;
use crate::utils::verify::is_valid_auth_string;

pub async fn signup(state: web::Data<AppState>, req: web::Form<UserForm>) -> impl Responder {
    let is_valid = is_valid_auth_string(&req.username);

    if !is_valid {
        return HttpResponse::build(StatusCode::BAD_REQUEST)
            .body("Invalid username input");
    }

    let pool = &state.pool;
    let connection = pg_pool_handler(&pool).unwrap();

    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_bytes = req.password.as_bytes();
    // Argon2 hash as string to store in DB
    let password_hash = argon2.hash_password(password_bytes, &salt).unwrap().to_string();

    let _user = diesel::insert_into(users::table)
        .values(NewUser {
            username: req.username.clone(),
            password: password_hash,
        })
        .get_result::<User>(&connection);
    let _user = match _user {
        Ok(user) => user,
        Err(_err) => {
             return HttpResponse::build(StatusCode::CONFLICT)
                .body("User already exists");
        }
    };
    HttpResponse::build(StatusCode::OK)
        .body("Successfully signed up")
}

pub async fn login(
    session: Session,
    state: web::Data<AppState>, 
    req: web::Query<UserForm>,
) -> impl Responder {
    let pool = &state.pool;
    let connection = pg_pool_handler(&pool).unwrap();

    let argon2 = Argon2::default();
    let password_bytes = req.password.as_bytes();

    let user = users::table.filter(users::username.eq(req.username.clone()))
        .get_result::<User>(&connection);
    let user = match user {
        Ok(user) => user,
        Err(_err) => {
            return HttpResponse::build(StatusCode::UNAUTHORIZED)
            .body("User not found");
        }
    };

    let parsed_hash = PasswordHash::new(&user.password).unwrap();

    let passwords_match = argon2.verify_password(password_bytes, &parsed_hash).is_ok();
    match passwords_match {
        true => {},
        false => {
            return HttpResponse::build(StatusCode::UNAUTHORIZED)
                .body("Incorrect password")
        }
    }

    session.insert("id", user.id).unwrap();

    HttpResponse::build(StatusCode::OK)
        .body("Successfully logged in")
}

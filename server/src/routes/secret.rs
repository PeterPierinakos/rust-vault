use actix_web::{web, HttpResponse, Responder};
use crate::models::secret::DeleteUserSecretForm;
use http::StatusCode;
use crate::config::VERY_SECRET_PRIVATE_KEY;
use diesel::prelude::*;
use crate::schema::users;
use crate::models::user::User;
use crate::structs::app_state::AppState;
use crate::utils::db::pg_pool_handler;

pub async fn delete_account(state: web::Data<AppState>, req: web::Form<DeleteUserSecretForm>) -> impl Responder {
	let pool = &state.pool;
    let connection = pg_pool_handler(&pool).unwrap();

	if req.secret_key != VERY_SECRET_PRIVATE_KEY {
		return HttpResponse::build(StatusCode::FORBIDDEN)
			.body("Wrong secret key");
	}

	let deleted = diesel::delete(users::table.filter(users::username.eq(req.target_username.clone())))
		.get_result::<User>(&connection);

	match deleted {
		Ok(_user) => {},
		Err(_err) => {
			return HttpResponse::build(StatusCode::BAD_REQUEST)
				.body("User deletion failed. Maybe user wasn't found?")
		}
	}

	HttpResponse::build(StatusCode::OK)
		.body("Success")
}
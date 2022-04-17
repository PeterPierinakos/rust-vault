use crate::utils::db::PgPool;

pub struct AppState {
    pub pool: PgPool
}
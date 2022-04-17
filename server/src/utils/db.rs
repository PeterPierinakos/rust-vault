use diesel::pg::PgConnection;

use diesel::r2d2::{ Pool, PooledConnection, ConnectionManager, PoolError };
use crate::config::DATABASE_URL;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> PgPool {
    let database_url = DATABASE_URL; 
    init_pool(&database_url).expect("Failed to create pool")
}

fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::new(manager)
}

pub fn pg_pool_handler(pool: &PgPool) -> Result<PgPooledConnection, PoolError> {
    let _pool = pool.get().unwrap();
    Ok(_pool)
}

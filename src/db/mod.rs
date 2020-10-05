pub mod models;
pub mod schema;

use actix_web::{web, HttpResponse, Result};
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};

// Embed and run migrations
embed_migrations!();

pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;
pub type MySqlPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

pub fn mysql_pool_handler(
    pool: web::Data<MysqlPool>,
) -> Result<MySqlPooledConnection, HttpResponse> {
    pool.get()
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

// TODO: Better error handling
pub fn init(database_url: &str) -> Result<MysqlPool, PoolError> {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Run embedded database migrations
    embedded_migrations::run(&pool.get().expect("Failed to migrate."))
        .expect("Failed to run database migrations.");

    Ok(pool)
}

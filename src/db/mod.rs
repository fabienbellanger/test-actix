//! Database module

pub mod schema;

use crate::errors::AppError;
use actix_web::{web, Result};
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};

// Embed and run migrations
embed_migrations!();

pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;
pub type MySqlPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

pub fn mysql_pool_handler(pool: web::Data<MysqlPool>) -> Result<MySqlPooledConnection, AppError> {
    pool.get().map_err(|_| AppError::InternalError {
        message: "Database error".to_owned(),
    })
}

pub fn init(database_url: &str) -> Result<MysqlPool, PoolError> {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = Pool::builder().build(manager)?;

    // Run embedded database migrations
    embedded_migrations::run_with_output(
        &pool.get().expect("Failed to run database migrations."),
        &mut std::io::stdout(),
    )
    .expect("Failed to run database migrations.");

    Ok(pool)
}

pub mod models;
pub mod schema;

use crate::{MySqlPooledConnection, MysqlPool};
use actix_web::{web, HttpResponse, Result};

pub fn mysql_pool_handler(
    pool: web::Data<MysqlPool>,
) -> Result<MySqlPooledConnection, HttpResponse> {
    pool.get()
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

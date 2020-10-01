use crate::db::models::{NewUser, User};
use crate::{MySqlPooledConnection, MysqlPool};
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse, Result};

fn mysql_pool_handler(pool: web::Data<MysqlPool>) -> Result<MySqlPooledConnection, HttpResponse> {
    pool.get()
        .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

#[get("/users")]
pub async fn get_users(_req: HttpRequest, pool: web::Data<MysqlPool>) -> Result<HttpResponse> {
    let mysql_pool = mysql_pool_handler(pool)?;

    Ok(HttpResponse::Ok().json(crate::db::models::UserList::list(&mysql_pool)))
}

#[post("/users")]
pub async fn create_user(
    form: web::Json<NewUser>,
    pool: web::Data<MysqlPool>,
) -> Result<HttpResponse, Error> {
    let mysql_pool = mysql_pool_handler(pool)?;

    let user =
        User::create(&mysql_pool, form.lastname.clone(), form.lastname.clone()).map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(user))
}

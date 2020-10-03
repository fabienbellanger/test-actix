use crate::db;
use crate::db::models::{NewUser, User};
use crate::db::MysqlPool;
use actix_web::{delete, get, post, web, Error, HttpRequest, HttpResponse, Result};

// TODO: Gérer avec des AppError

#[get("/users")]
pub async fn get_users(_req: HttpRequest, pool: web::Data<MysqlPool>) -> Result<HttpResponse> {
    let mysql_pool = db::mysql_pool_handler(pool)?;

    Ok(HttpResponse::Ok().json(crate::db::models::UserList::list(&mysql_pool)))
}

#[get("/users/{id}")]
pub async fn get_user_by_id(
    web::Path(id): web::Path<String>,
    pool: web::Data<MysqlPool>,
) -> Result<HttpResponse> {
    let mysql_pool = db::mysql_pool_handler(pool)?;

    let user = web::block(move || User::get_by_id(&mysql_pool, id))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    Ok(HttpResponse::Ok().json(user))
}

#[post("/users")]
pub async fn create_user(
    form: web::Json<NewUser>,
    pool: web::Data<MysqlPool>,
) -> Result<HttpResponse, Error> {
    let mysql_pool = db::mysql_pool_handler(pool)?;

    let user = web::block(move || User::create(&mysql_pool, form.into_inner()))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(user))
}

#[delete("/users/{id}")]
pub async fn delete_user_by_id(
    web::Path(id): web::Path<String>,
    pool: web::Data<MysqlPool>,
) -> Result<HttpResponse> {
    let mysql_pool = db::mysql_pool_handler(pool)?;

    let num_deleted = web::block(move || User::delete(&mysql_pool, id))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    match num_deleted {
        0 => Ok(HttpResponse::NotFound().finish()),
        _ => Ok(HttpResponse::Ok().finish()),
    }
}

use crate::db;
use crate::db::models::{NewUser, User};
use crate::MysqlPool;
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse, Result};

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

    let user = web::block(move || {
        User::create(&mysql_pool, form.lastname.clone(), form.firstname.clone())
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(user))
}

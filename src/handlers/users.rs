use crate::db;
use crate::db::models::{User, UserJson};
use crate::db::MysqlPool;
use actix_web::{web, Error, HttpRequest, HttpResponse, Result};

// TODO: Gérer avec des AppError

// Route: "/users"
// curl http://localhost:8089/v1/users
pub async fn get_users(pool: web::Data<MysqlPool>, _req: HttpRequest) -> Result<HttpResponse> {
    let mysql_pool = db::mysql_pool_handler(pool)?;

    Ok(HttpResponse::Ok().json(crate::db::models::UserList::list(&mysql_pool)))
}

// Route: "/users/{id}
// curl http://localhost:8089/v1/users/<uuid>
pub async fn get_by_id(
    pool: web::Data<MysqlPool>,
    web::Path(id): web::Path<String>,
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

// Route: "/users"
// curl -H "Content-Type: application/json" -X POST http://127.0.0.1:8089/v1/users -d '{"lastname":"Bellanger", "firstname":"Fabien"}'
pub async fn create(
    pool: web::Data<MysqlPool>,
    form: web::Json<UserJson>,
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

// Route: "/users/{id}"
// curl -X DELETE http://127.0.0.1:8089/v1/users/<uuid>
pub async fn delete(
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

// Route: "/users/{id}"
// curl -H "Content-Type: application/json" -X PUT http://127.0.0.1:8089/v1/users/<uuid> -d '{"lastname":"Bellanger", "firstname":"Fabien"}'
pub async fn update(
    pool: web::Data<MysqlPool>,
    web::Path(id): web::Path<String>,
    form: web::Json<UserJson>,
) -> Result<HttpResponse> {
    let mysql_pool = db::mysql_pool_handler(pool)?;

    let user = web::block(move || User::update(&mysql_pool, id, form.into_inner()))
        .await
        .map_err(|e| match e.to_string().as_str() {
            "NotFound" => HttpResponse::NotFound().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        })?;

    Ok(HttpResponse::Ok().json(user))
}

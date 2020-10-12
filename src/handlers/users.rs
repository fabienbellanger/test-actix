use crate::db;
use crate::db::MysqlPool;
use crate::errors::AppError;
use crate::models::auth::JWT;
use crate::models::user::{Login, LoginResponse, NewUser, User, UserList};
use crate::AppState;
use actix_web::{web, HttpRequest, HttpResponse, Result};
use chrono::prelude::*;
use log::error;

// Route: POST "/login"
// curl -H "Content-Type: application/json" -X POST http://127.0.0.1:8089/v1/login \
// -d '{"email":"fabien.bellanger3@test.com", "password": "0000"}'
pub async fn login(
    pool: web::Data<MysqlPool>,
    data: web::Data<AppState>,
    form: web::Json<Login>,
) -> Result<HttpResponse, AppError> {
    let mysql_pool = db::mysql_pool_handler(pool)?;

    let user = web::block(move || User::login(&mysql_pool, form.into_inner()))
        .await
        .map_err(|e| {
            error!("{}", e);
            AppError::Unauthorized {}
        })?;

    // Génération du token
    // -------------------
    let secret = &data.jwt_secret_key;
    let token = JWT::generate(
        user.id.to_owned(),
        user.lastname.to_owned(),
        user.firstname.to_owned(),
        user.email.to_owned(),
        secret.to_owned(),
    );

    match token {
        Ok(token) => {
            let expires_at = chrono::NaiveDateTime::from_timestamp(token.1, 0);
            let expires_at: DateTime<Utc> = DateTime::from_utc(expires_at, Utc);

            Ok(HttpResponse::Ok().json(LoginResponse {
                lastname: user.lastname.to_owned(),
                firstname: user.firstname.to_owned(),
                email: user.email.to_owned(),
                token: token.0,
                expires_at: expires_at.to_rfc3339_opts(SecondsFormat::Secs, true), // format("%Y-%m-%d %H:%M:%S").to_string(),
            }))
        }
        _ => Err(AppError::Unauthorized {}),
    }
}

// Route: POST "/register"
// curl -H "Content-Type: application/json" -X POST http://127.0.0.1:8089/v1/register \
// -d '{"lastname":"Bellanger", "firstname":"Fabien", "email":"fabien.bellanger3@test.com", "password": "0000"}'
pub async fn create(
    pool: web::Data<MysqlPool>,
    form: web::Json<NewUser>,
) -> Result<HttpResponse, AppError> {
    let mysql_pool = db::mysql_pool_handler(pool)?;

    let user = web::block(move || User::create(&mysql_pool, form.into_inner()))
        .await
        .map_err(|e| {
            error!("{}", e);
            AppError::InternalError {
                message: "Error during user creation".to_owned(),
            }
        })?;

    Ok(HttpResponse::Ok().json(user))
}

// Route: GET "/users"
// curl http://localhost:8089/v1/users -H 'Authorization: Bearer '
pub async fn get_users(
    pool: web::Data<MysqlPool>,
    _req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let mysql_pool = db::mysql_pool_handler(pool)?;

    error!("Users list");

    let users = web::block(move || UserList::list(&mysql_pool))
        .await
        .map_err(|e| {
            error!("{}", e);
            AppError::InternalError {
                message: "Error while retrieving users list".to_owned(),
            }
        })?;
    Ok(HttpResponse::Ok().json(users))
}

// Route: GET "/users/{id}
// curl http://localhost:8089/v1/users/<uuid>
pub async fn get_by_id(
    pool: web::Data<MysqlPool>,
    web::Path(id): web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let mysql_pool = db::mysql_pool_handler(pool)?;

    let user = web::block(move || User::get_by_id(&mysql_pool, id))
        .await
        .map_err(|e| {
            error!("{}", e);
            AppError::InternalError {
                message: "Error while retrieving a user's information".to_owned(),
            }
        })?;
    Ok(HttpResponse::Ok().json(user))
}

// Route: DELETE "/users/{id}"
// curl -X DELETE http://127.0.0.1:8089/v1/users/<uuid>
pub async fn delete(
    web::Path(id): web::Path<String>,
    pool: web::Data<MysqlPool>,
) -> Result<HttpResponse, AppError> {
    let mysql_pool = db::mysql_pool_handler(pool)?;

    let num_deleted = web::block(move || User::delete(&mysql_pool, id))
        .await
        .map_err(|e| {
            error!("{}", e);
            AppError::InternalError {
                message: "Error during user deletion".to_owned(),
            }
        })?;

    match num_deleted {
        0 => Err(AppError::NotFound {
            message: "User not found".to_owned(),
        }),
        _ => Ok(HttpResponse::Ok().finish()),
    }
}

// Route: PUT "/users/{id}"
// curl -H "Content-Type: application/json" -X PUT http://127.0.0.1:8089/v1/users/<uuid> -d '{"lastname":"Bellanger", "firstname":"Fabien"}'
pub async fn update(
    pool: web::Data<MysqlPool>,
    web::Path(id): web::Path<String>,
    form: web::Json<NewUser>,
) -> Result<HttpResponse, AppError> {
    let mysql_pool = db::mysql_pool_handler(pool)?;

    let user = web::block(move || User::update(&mysql_pool, id, form.into_inner()))
        .await
        .map_err(|e| match e.to_string().as_str() {
            "NotFound" => AppError::NotFound {
                message: "User not found".to_owned(),
            },
            _ => AppError::InternalError {
                message: "Error during user update".to_owned(),
            },
        })?;

    Ok(HttpResponse::Ok().json(user))
}

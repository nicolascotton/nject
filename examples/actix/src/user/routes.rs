use super::{
    super::Prov,
    models::{CreateUser, User},
    service::UserService,
};
use actix_web::{
    delete, get, post, put,
    web::{self, scope, Json, Path},
    Either, HttpResponse, Responder, Result, Scope,
};

pub fn create_scope() -> Scope {
    scope("/api/users")
        .service(get_user)
        .service(create_user)
        .service(update_user)
        .service(delete_user)
}

#[get("/{user_id}")]
async fn get_user(provider: Prov, user_id: Path<i64>) -> Result<impl Responder> {
    let service = provider.provide::<UserService>();
    let result = service.get(*user_id).await;
    match result {
        Ok(user) => Ok(Either::Left(web::Json(user))),
        Err(error) => Ok(Either::Right((
            web::Json(error),
            actix_web::http::StatusCode::BAD_REQUEST,
        ))),
    }
}

#[post("/")]
async fn create_user(provider: Prov, user: Json<CreateUser>) -> Result<impl Responder> {
    let service = provider.provide::<UserService>();
    let result = service.create(&user).await;
    match result {
        Ok(user) => Ok(Either::Left(web::Json(user))),
        Err(error) => Ok(Either::Right((
            web::Json(error),
            actix_web::http::StatusCode::BAD_REQUEST,
        ))),
    }
}

#[put("/")]
async fn update_user(provider: Prov, user: Json<User>) -> Result<impl Responder> {
    let service = provider.provide::<UserService>();
    let result = service.update(&user).await;
    match result {
        Ok(_) => Ok(Either::Left(HttpResponse::Ok())),
        Err(error) => Ok(Either::Right((
            web::Json(error),
            actix_web::http::StatusCode::BAD_REQUEST,
        ))),
    }
}

#[delete("/{user_id}")]
async fn delete_user(provider: Prov, user_id: Path<i64>) -> Result<impl Responder> {
    let service = provider.provide::<UserService>();
    let result = service.delete(*user_id).await;
    match result {
        Ok(_) => Ok(Either::Left(HttpResponse::Ok())),
        Err(error) => Ok(Either::Right((
            web::Json(error),
            actix_web::http::StatusCode::BAD_REQUEST,
        ))),
    }
}

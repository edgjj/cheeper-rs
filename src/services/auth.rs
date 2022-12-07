use actix_identity::Identity;
use actix_web::{error, get, post, web, Error, HttpMessage, HttpRequest, HttpResponse, Responder};
use opensearch::{IndexParts, http::response::Response};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use super::ServerState;
use crate::dto::User;

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

// we may share this between different requests as long as data is same
#[derive(Deserialize)]

struct LoginRegisterRequest {
    username: String,
    password: String,
}

#[post("/register")]
async fn register_user(
    state: web::Data<ServerState>,
    req: web::Json<LoginRegisterRequest>,
) -> impl Responder {
    
    let client = &state.client;
    let req = req.into_inner();
    
    let salt = SaltString::generate(&mut OsRng);

    let pw_hash = match PasswordHash::generate(Argon2::default(), req.password, salt.as_str()){
        Err(_) => return HttpResponse::InternalServerError(),
        Ok(hash) => hash.to_string()
    };
    
    let new_user = User::new(req.username.clone(), pw_hash);

    match client
        .index(IndexParts::Index("users"))
        .body(new_user)
        .send()
        .await {
        Ok(_) => HttpResponse::Ok(),
        _ => HttpResponse::ServiceUnavailable()
    }
}

#[post("/login")] // web::Json<LoginRegisterRequest>
async fn login_user(
    state: web::Data<ServerState>,
    req: web::Json<LoginRegisterRequest>,
    plain_req: HttpRequest,
) -> impl Responder {
    let client = &state.client;

    let login_identity = Identity::login(&plain_req.extensions(), req.into_inner().username);
    match login_identity {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(error) => HttpResponse::InternalServerError()
            .body(format!("Internal error: {}", error.to_string())), //HttpResponse::from_error(error),
    }
}

#[post("/logout")]
async fn logout_user(user: Identity) -> impl Responder {
    user.logout();
    HttpResponse::Ok()
}

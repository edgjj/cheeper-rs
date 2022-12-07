use actix_identity::Identity;
use actix_web::{error, get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};

use serde::Deserialize;

use super::ServerState;

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
    // set cookie ?
    HttpResponse::Ok()
}

#[post("/login")] // web::Json<LoginRegisterRequest>
async fn login_user(
    state: web::Data<ServerState>,
    req: web::Json<LoginRegisterRequest>,
    plain_req: HttpRequest,
) -> impl Responder {
    
    let login_identity = Identity::login(&plain_req.extensions(), req.username.clone());
    match (login_identity) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(error) => HttpResponse::InternalServerError()
            .json(format!("Internal error: {}", error.to_string())), //HttpResponse::from_error(error),
    }
}

#[post("/logout")]
async fn logout_user(user: Identity) -> impl Responder {
    user.logout();
    HttpResponse::Ok()
}

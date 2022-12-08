use actix_identity::Identity;
use actix_web::{
    error, http::StatusCode, post, web, Error, HttpMessage, HttpRequest, HttpResponse, Responder,
};
use opensearch::IndexParts;
use serde::Deserialize;
use serde_partial::SerializePartial;

use super::users::{get_user, UserSearchType};
use crate::server::State;

use crate::dto::User;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordVerifier, SaltString},
    Argon2,
};

// we may share this between different requests as long as data is same
#[derive(Deserialize)]
struct LoginRegisterRequest {
    username: String,
    password: String,
}

#[post("/register")]
async fn register_user(
    state: web::Data<State>,
    req: web::Json<LoginRegisterRequest>,
) -> impl Responder {
    let client = &state.client;
    let req = req.into_inner();

    match get_user(client, &req.username, UserSearchType::ByName).await {
        Ok(_) => return HttpResponse::Ok().body("Already registered."),
        Err(e) if e.as_response_error().status_code() == StatusCode::NOT_FOUND => (),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    }

    let salt = SaltString::generate(&mut OsRng);

    let pw_hash = match PasswordHash::generate(Argon2::default(), req.password, salt.as_str()) {
        Err(_) => return HttpResponse::InternalServerError().finish(),
        Ok(hash) => hash.to_string(),
    };

    let new_user = User::new(req.username, pw_hash);

    match client
        .index(IndexParts::Index("users"))
        .body(&new_user)
        .send()
        .await
    {
        Ok(response) => {
            if response.status_code().is_success() {
                let no_pw = new_user.without_fields(|u| [u.pw_hash]);
                HttpResponse::Ok().json(no_pw)
            } else {
                HttpResponse::Unauthorized().finish()
            }
        }
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/login")] // web::Json<LoginRegisterRequest>
async fn login_user(
    state: web::Data<State>,
    req: web::Json<LoginRegisterRequest>,
    plain_req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let client = &state.client;
    let user = get_user(client, &req.username, UserSearchType::ByName).await?;

    let parsed_hash = PasswordHash::new(&user.pw_hash).unwrap();

    if Argon2::default()
        .verify_password(req.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        Err(error::ErrorUnauthorized("Invalid credentials."))
    } else {
        let login_identity = Identity::login(&plain_req.extensions(), user.id.to_string());

        match login_identity {
            Ok(_) => Ok(HttpResponse::Ok().finish()),
            Err(_error) => Err(error::ErrorInternalServerError("Failed to make identity")), 
        }
    }
}

#[post("/logout")]
async fn logout_user(user: Identity) -> impl Responder {
    user.logout();
    HttpResponse::Ok()
}

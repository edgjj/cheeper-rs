use actix_identity::Identity;
use actix_web::{
    error, http::StatusCode, post, web, Error, HttpMessage, HttpRequest, HttpResponse, Responder,
};
use opensearch::{IndexParts, OpenSearch, SearchParts};
use serde::Deserialize;
use serde_json::Value;
use serde_partial::SerializePartial;

use super::ServerState;
use crate::dto::User;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use log::debug;

// we may share this between different requests as long as data is same
#[derive(Deserialize)]

struct LoginRegisterRequest {
    username: String,
    password: String,
}
async fn get_user(client: &OpenSearch, username: &String) -> Result<User, actix_web::Error> {
    // Result<String, actix_web::HttpResponse>{

    match client
        .search(SearchParts::Index(&["users"]))
        .q(format!("username:{}", username).as_str())
        .send()
        .await
    {
        Ok(response) => {
            let mut search_result = response.json::<Value>().await.unwrap();

            if search_result["hits"]["total"]["value"].as_i64().unwrap() == 0 {
                return Err(error::ErrorUnauthorized("")); // error
            }

            let user_json = search_result["hits"]["hits"][0]["_source"].take();
            let user: User = serde_json::from_value(user_json).unwrap();

            Ok(user)
        }

        Err(_) => Err(error::ErrorInternalServerError("")),
    }
}

#[post("/register")]
async fn register_user(
    state: web::Data<ServerState>,
    req: web::Json<LoginRegisterRequest>,
) -> impl Responder {
    let client = &state.client;
    let req = req.into_inner();

    match get_user(client, &req.username).await {
        Ok(_) => return HttpResponse::Ok().body("Already registered."),
        Err(e) if e.as_response_error().status_code() == StatusCode::UNAUTHORIZED => (),
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
            if !response.status_code().is_success() {
                return HttpResponse::Unauthorized().finish();
            }

            let no_pw = new_user.without_fields(|u| [u.pw_hash]);
            HttpResponse::Ok().json(no_pw)
        }
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/login")] // web::Json<LoginRegisterRequest>
async fn login_user(
    state: web::Data<ServerState>,
    req: web::Json<LoginRegisterRequest>,
    plain_req: HttpRequest,
) -> Result<HttpResponse, Error> {
    let client = &state.client;
    let user = get_user(client, &req.username).await?;

    let parsed_hash = PasswordHash::new(&user.pw_hash).unwrap();

    if Argon2::default()
        .verify_password(req.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return Err(error::ErrorUnauthorized("Invalid credentials."));
    }
    
    let login_identity = Identity::login(&plain_req.extensions(), user.id.to_string());

    match login_identity {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_error) => Err(error::ErrorInternalServerError("Failed to make identity")), //HttpResponse::from_error(error),
    }
}

#[post("/logout")]
async fn logout_user(user: Identity) -> impl Responder {
    user.logout();
    HttpResponse::Ok()
}

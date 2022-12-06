use actix_web::{get, post, web, HttpResponse, Responder};
use std::time::SystemTime;

use serde::Deserialize;

use super::ServerState;
use crate::dto::Message;

#[derive(Deserialize)]
struct MakeFriendsRequest {
    friend_id: u64,
}

#[post("/users/{user_id}/friends")]
async fn make_friends(
    state: web::Data<ServerState>,
    req: web::Json<MakeFriendsRequest>,
) -> impl Responder {
    HttpResponse::Ok()
}

#[derive(Deserialize)]
struct FriendsRequest {
    count: String, // this should be empty as long as ?count == ?count=
}

#[get("/users/{user_id}/friends")]
async fn get_friend(
    state: web::Data<ServerState>,
    req: web::Json<MakeFriendsRequest>,
) -> impl Responder {
    HttpResponse::Ok()
}

use actix_web::{get, post, web, HttpResponse, Responder};
use std::time::SystemTime;

use serde::Deserialize;

use super::ServerState;
use crate::dto::Message;

#[derive(Deserialize)]
struct MessagesDateSpan {
    date_start: Option<SystemTime>,
    date_end: Option<SystemTime>,
}

#[get("/messages/get/{user_id}")]
pub async fn index_messages(
    state: web::Data<ServerState>,
    path: web::Path<u64>,
    query: web::Query<MessagesDateSpan>,
) -> impl Responder {
    let user_id = path.into_inner();
    let date_info = query.into_inner();

    // do state.client interaction

    // custom responder
    HttpResponse::Ok()
}

#[derive(Deserialize)]
struct SendMessageRequest {
    text: String,
}

#[post("/messages/send")]
pub async fn send_message(
    state: web::Data<ServerState>,
    req: web::Json<SendMessageRequest>,
) -> impl Responder {
    // do state.client interaction

    // custom responder (return dto::Message as inserted in ES)
    HttpResponse::Ok()
}

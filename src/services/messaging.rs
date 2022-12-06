use actix_web::{web, get, post, Responder, HttpResponse, rt::System};
use std::time::SystemTime;

use serde::Deserialize;

use super::ServerState;
use crate::dto::Message;

#[derive(Deserialize)]
struct MessagesDateSpan{
    date_start: Option<SystemTime>,
    date_end: Option<SystemTime>
}

#[get("/messages/get/{user_id}")]
async fn index_messages(state: web::Data<ServerState>, path: web::Path<u64>, query: web::Query<MessagesDateSpan>) -> impl Responder{
    let user_id = path.into_inner();
    let date_info = query.into_inner();

    // custom user responder
    HttpResponse::Ok()
}

#[derive(Deserialize)]
struct SendMessageRequest{
    text: String,
}

#[post("/messages/send")]
async fn send_message(state: web::Json<SendMessageRequest>) -> impl Responder{
    HttpResponse::Ok()
}
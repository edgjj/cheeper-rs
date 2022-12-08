use actix_identity::Identity;
use actix_web::{get, post, web, HttpResponse, Responder};
use opensearch::{IndexParts, OpenSearch, SearchParts};
use serde_json::Value;

use serde::Deserialize;

use super::ServerState;
use crate::dto::Message;

use log::debug;

#[derive(Deserialize)]
struct MessagesDateSpan {
    date_start: Option<String>,
    date_end: Option<String>,
}

#[get("/messages/get/{user_id}")]
async fn index_messages(
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
async fn send_message(
    state: web::Data<ServerState>,
    req: web::Json<SendMessageRequest>,
    identity: Identity,
) -> impl Responder {
    let req = req.into_inner();
    let client = &state.client;

    let new_message = Message::new(identity.id().unwrap(), req.text);

    match client
        .index(IndexParts::Index("messages"))
        .body(&new_message)
        .send()
        .await
    {
        Ok(response) => {
            if !response.status_code().is_success() {
                HttpResponse::InternalServerError().body("Failed to send message")
            } else {
                HttpResponse::Ok().json(new_message)
            }
        }
        Err(_) => HttpResponse::ServiceUnavailable().finish(),
    }
}

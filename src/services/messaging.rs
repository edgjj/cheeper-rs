use actix_identity::Identity;

use actix_web::{error, get, post, web, Error, HttpResponse, Responder};
use opensearch::{IndexParts, SearchParts};

use serde::Deserialize;
use uuid::Uuid;

use serde_json::json;

use super::users::{get_user, UserSearchType};

use crate::dto::Message;
use crate::server::State;

use super::response_utils;

#[derive(Deserialize)]
struct MessagesDateSpan {
    date_start: Option<String>,
    date_end: Option<String>,
}

#[get("/messages/{username_or_id}")]
async fn index_messages(
    state: web::Data<State>,
    path: web::Path<String>,
    query: Option<web::Json<MessagesDateSpan>>, // optional JSON with optional values
) -> Result<HttpResponse, Error> {
    let client = &state.client;
    let username_or_id = path.into_inner();

    // check if id is valid and get actual id of not
    let user_id = match Uuid::try_parse(&username_or_id) {
        Ok(_) => username_or_id,
        Err(_) => get_user(client, &username_or_id, UserSearchType::ByName)
            .await?
            .id
            .to_string(),
    };

    let mut date_query = "".to_owned();

    if let Some(date_info) = query {
        let date_info = date_info.into_inner();

        date_query = match (date_info.date_start, date_info.date_end) {
            (Some(start), None) => format!("AND created_at:[{start} TO now]"),
            (None, Some(end)) => format!("AND created_at:[* TO {end}]"),
            (Some(start), Some(end)) => format!("AND created_at:[{start} TO {end}]"),
            _ => date_query,
        };
    }

    match client
        .search(SearchParts::Index(&["messages"]))
        .q(format!("author_id:{user_id} {date_query}").as_str())
        .filter_path(&["hits.total.value", "hits.hits._source"]) //
        .send()
        .await
    {
        Ok(response) => {
            let parsed = response_utils::parse_message_search(response).await?;

            Ok(HttpResponse::Ok().json(json!({
                "count": parsed.len(),
                "items": parsed
            })))
        }
        Err(_) => Err(error::ErrorInternalServerError("")),
    }
}

#[derive(Deserialize)]
struct SendMessageRequest {
    text: String,
}

#[post("/messages/send")]
async fn send_message(
    state: web::Data<State>,
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
            if response.status_code().is_success() {
                HttpResponse::Ok().json(new_message)
            } else {
                HttpResponse::InternalServerError().body("Failed to send message")
            }
        }
        Err(_) => HttpResponse::ServiceUnavailable().finish(),
    }
}

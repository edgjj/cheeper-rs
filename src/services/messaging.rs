use actix_identity::Identity;
use actix_web::{error, get, post, web, Error, HttpResponse, Responder};
use opensearch::{IndexParts, SearchParts};

use serde::Deserialize;
use uuid::Uuid;

use serde_json::json;
use serde_json::Value;

use super::tools::*;
use super::ServerState;

use crate::dto::Message;

#[derive(Deserialize)]
struct MessagesDateSpan {
    date_start: Option<String>,
    date_end: Option<String>,
}

#[get("/messages/{username_or_id}")]
async fn index_messages(
    state: web::Data<ServerState>,
    path: web::Path<String>,
    query: web::Query<MessagesDateSpan>,
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

    let date_info = query.into_inner();

    match client
        .search(SearchParts::Index(&["messages"]))
        .q(format!("author_id:{}", user_id).as_str())
        .filter_path(&["hits.total.value", "hits.hits._source"]) //
        .send()
        .await
    {
        Ok(response) => {
            let mut search_result: Value = response.json().await.unwrap();
            let num_messages = search_result["hits"]["total"]["value"].as_i64().unwrap();

            let mut messages_vec = json!([]);

            // map results
            if num_messages != 0 {
                let hits = search_result["hits"]["hits"].as_array_mut().unwrap();
                messages_vec = hits
                    .iter_mut()
                    .map(|v| v.as_object_mut().unwrap().remove("_source").unwrap())
                    .collect();
            }

            Ok(HttpResponse::Ok().json(json!({
                "count": num_messages,
                "items": messages_vec
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

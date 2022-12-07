use actix_web::http::StatusCode;
use opensearch::{
    indices::Indices, indices::IndicesCreateParts, indices::IndicesExistsParts, OpenSearch,
};
use serde_json::json;

pub async fn create_indices(client: &OpenSearch) {
    create_users_index(client).await;
    create_messages_index(client).await;
}

async fn create_users_index(client: &OpenSearch) {
    let indices = client.indices();

    let exists = indices
        .exists(IndicesExistsParts::Index(&["users"]))
        .send()
        .await
        .unwrap();

    if exists.status_code().is_success() {
        return;
    }

    indices
        .create(IndicesCreateParts::Index("users"))
        .body(json!({
            "mappings": {
                "properties": {
                    "id": { "type" : "text" },
                    "author_id": { "type": "text" },
                    "created_at": { "type": "date"},
                    "text": { "type": "text"},
                }
            }
        }))
        .send()
        .await
        .unwrap(); // unwrap to panic! in case if index creation failed
}

async fn create_messages_index(client: &OpenSearch) {
    let indices = client.indices();

    let exists = indices
        .exists(IndicesExistsParts::Index(&["messages"]))
        .send()
        .await
        .unwrap();

    // check if not 404
    if exists.status_code().is_success() {
        return;
    }

    indices
        .create(IndicesCreateParts::Index("users"))
        .body(json!({
            "mappings": {
                "properties": {
                    "id": { "type" : "text" },
                    "username": { "type": "text" },
                    "pw_hash": { "type": "text" },
                    "created_at": { "type": "date"},
                    "friend_list": { "type": "text"},
                }
            }
        }))
        .send()
        .await
        .unwrap(); // unwrap to panic! in case if index creation failed
}

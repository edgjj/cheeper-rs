use opensearch::{indices::IndicesCreateParts, indices::IndicesExistsParts, OpenSearch};
use serde_json::json;

pub async fn create_indices(client: &OpenSearch) {
    make_index(
        client,
        "users",
        json!({
            "id": { "type" : "text" },
            "author_id": { "type": "text" },
            "created_at": { "type": "date" },
            "text": { "type": "text" },
            "pw_hash": { "type": "text" },
        }),
    )
    .await;

    make_index(
        client,
        "messages",
        json!({
            "id": { "type" : "text" },
            "username": { "type": "text" },
            "created_at": { "type": "date" },
            "friend_list": { "type": "text" },
        }),
    )
    .await;
}

async fn make_index(client: &OpenSearch, name: &str, mapping: serde_json::Value) {
    let indices = client.indices();

    let exists = indices
        .exists(IndicesExistsParts::Index(&[name]))
        .send()
        .await
        .unwrap();

    if exists.status_code().is_success() {
        return;
    }

    indices
        .create(IndicesCreateParts::Index(name))
        .body(json!({
            "mappings": {
                "properties": mapping
            }
        }))
        .send()
        .await
        .unwrap(); // unwrap to panic! in case if index creation failed
}

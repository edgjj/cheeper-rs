use crate::dto::User;
use actix_web::error;
use opensearch::{OpenSearch, SearchParts};
use serde_json::Value;

pub enum UserSearchType {
    ByName,
    ById,
}

pub async fn get_user(
    client: &OpenSearch,
    username_or_id: &String,
    tag: UserSearchType,
) -> Result<User, actix_web::Error> {
    let search_query = match tag {
        ByName => format!("username:{}", username_or_id),
        ById => format!("id:{}", username_or_id),
    };

    match client
        .search(SearchParts::Index(&["users"]))
        .q(search_query.as_str())
        .filter_path(&["hits.total.value", "hits.hits._source"])
        .send()
        .await
    {
        Ok(response) => {
            let mut search_result = response.json::<Value>().await.unwrap();

            if search_result["hits"]["total"]["value"].as_i64().unwrap() == 0 {
                Err(error::ErrorNotFound("User not found")) // error
            } else {
                let user_json = search_result["hits"]["hits"][0]["_source"].take();
                let user: User = serde_json::from_value(user_json).unwrap();

                Ok(user)
            }
        }

        Err(_) => Err(error::ErrorInternalServerError("")),
    }
}

use actix_identity::Identity;
use actix_web::{error, get, post, web, Error, HttpResponse};
use opensearch::{OpenSearch, SearchParts, UpdateByQueryParts};

use serde::Deserialize;
use serde_json::json;
use serde_partial::SerializePartial;
use uuid::Uuid;

use crate::dto::User;
use crate::server::State;

use super::response_utils;

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
        UserSearchType::ByName => format!("username:{}", username_or_id),
        UserSearchType::ById => format!("id:{}", username_or_id),
    };

    match client
        .search(SearchParts::Index(&["users"]))
        .q(search_query.as_str())
        .filter_path(&["hits.total.value", "hits.hits._source"])
        .send()
        .await
    {
        Ok(response) => response_utils::parse_user_search(response).await,

        Err(_) => Err(error::ErrorInternalServerError("")),
    }
}

#[derive(Deserialize)]
struct MakeFriendsRequest {
    friend_name_or_id: String,
}

#[post("/users/friend")]
async fn make_friends(
    state: web::Data<State>,
    req: web::Json<MakeFriendsRequest>,
    identity: Identity,
) -> Result<HttpResponse, Error> {
    let client = &state.client;

    let user_id = identity.id().unwrap();
    let mut user = get_user(client, &user_id, UserSearchType::ById).await?;

    let friend_user_id = match Uuid::try_parse(&req.friend_name_or_id) {
        Ok(id) => id,
        Err(_) => {
            get_user(client, &req.friend_name_or_id, UserSearchType::ByName)
                .await?
                .id
        }
    };

    if user.friend_list.contains(&friend_user_id) {
        return Ok(HttpResponse::Ok().body("Already has this friend"));
    }

    user.friend_list.push(friend_user_id);

    match client
        .update_by_query(UpdateByQueryParts::Index(&["users"]))
        .q(format!("id:{}", user_id).as_str())
        .body(json!({
            "script": {
                "source": "ctx._source.friend_list = params.new_friend_list",
                "lang": "painless",
                "params": {
                    "new_friend_list": user.friend_list
                }
            }
        }))
        .send()
        .await
    {
        Ok(response) => {
            if response.status_code().is_success() {
                let no_pw = user.without_fields(|u| [u.pw_hash]);
                Ok(HttpResponse::Ok().json(no_pw))
            } else {
                Ok(HttpResponse::Unauthorized().finish())
            }
        }
        _ => Err(error::ErrorInternalServerError("")),
    }
}

#[get("/users/{username}")]
async fn get_user_info(
    state: web::Data<State>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let client = &state.client;
    let username = path.into_inner();

    // check if id is valid and get actual id of not
    let user = get_user(client, &username, UserSearchType::ByName).await?;

    Ok(HttpResponse::Ok().json(user.without_fields(|u| [u.pw_hash])))
}

use actix_web::{get, post, web, Error, HttpResponse, Responder};

use serde::Deserialize;
use serde_partial::SerializePartial;

use super::tools;
use super::ServerState;

#[derive(Deserialize)]
struct MakeFriendsRequest {
    friend_id: u64,
}

#[post("/users/{user_id_or_name}/friends")]
async fn make_friends(
    state: web::Data<ServerState>,
    req: web::Json<MakeFriendsRequest>,
) -> impl Responder {
    HttpResponse::Ok()
}

#[get("/users/{username}")]
async fn get_user_info(
    state: web::Data<ServerState>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let client = &state.client;
    let username = path.into_inner();

    // check if id is valid and get actual id of not
    let user = tools::get_user(client, &username).await?;

    Ok(HttpResponse::Ok().json(user.without_fields(|u| [u.pw_hash])))
}

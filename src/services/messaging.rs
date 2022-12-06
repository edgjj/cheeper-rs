use actix_web::{web, get, post, Responder, HttpResponse};
use super::ServerState;


async fn get_message_by_id(state: web::Data<ServerState>) -> impl Responder{
    // custom user responder
    HttpResponse::Ok()
}

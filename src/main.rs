use actix_web::{App, HttpServer};
// use es_cheaper::dto::*;
use es_cheaper::services::ServerState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().app_data(ServerState::new("http://localhost:8080")))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

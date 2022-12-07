use actix_web::{App, HttpServer, web};
// use es_cheaper::dto::*;
use es_cheaper::services::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(|| { 
        App::new()
            .app_data(web::Data::new(ServerState::new("http://localhost:8080")))
            .service(messaging::index_messages)
            .service(messaging::send_message)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

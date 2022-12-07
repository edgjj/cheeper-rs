use actix_web::{web, App, HttpServer};
// use es_cheaper::dto::*;
use clap::Parser;
use es_cheaper::services::*;

#[derive(Parser)]
struct Args {
    #[arg(short = 'e', long = "engine")]
    engine_url: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let args = Args::parse();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(ServerState::new(args.engine_url.as_str())))
            .service(messaging::index_messages)
            .service(messaging::send_message)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

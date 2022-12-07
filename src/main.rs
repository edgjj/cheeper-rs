use actix_web::{App, HttpServer, web};
// use es_cheaper::dto::*;
use es_cheaper::services::*;
use clap::Parser;

#[derive(Parser)]
struct Args{
    #[arg(short = 'e', long = "engine")]
    engine_url: String
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

use actix_web::{web, App, HttpServer};
use clap::Parser;
use es_cheaper::{migrations, services::*};

#[derive(Parser)]
struct Args {
    #[arg(short = 'e', long = "engine")]
    engine_url: String,

    #[arg(short = 'u', long = "uname")]
    username: String,

    #[arg(short = 'p', long = "password")]
    password: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let args = Args::parse();
    let server_state = ServerState::new(args.engine_url, args.username, args.password);

    // create indices
    migrations::create_indices(&server_state.client).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server_state.clone()))
            .service(messaging::index_messages)
            .service(messaging::send_message)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

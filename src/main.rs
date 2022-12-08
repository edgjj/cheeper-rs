use actix_web::{cookie::Key, web, App, HttpServer};

use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};

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
    env_logger::init();

    let args = Args::parse();
    let server_state = ServerState::new(args.engine_url, args.username, args.password);

    // create indices
    migrations::create_indices(&server_state.client).await;

    let secret_key = Key::generate();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server_state.clone()))
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_name("es-cheeper".to_owned())
                    .cookie_secure(false)
                    .build(),
            )
            .service(auth::register_user)
            .service(auth::login_user)
            .service(auth::logout_user)
            .service(users::get_user_info)
            .service(users::make_friends)
            .service(messaging::index_messages)
            .service(messaging::send_message)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

use actix_web::dev::Server;
use elasticsearch::{http::transport::Transport, Elasticsearch};

pub struct ServerState {
    pub client: Elasticsearch,
}

impl ServerState {
    pub fn new(url: &str) -> ServerState {
        let transport = Transport::single_node(url).unwrap();
        let client = Elasticsearch::new(transport);

        ServerState { client: client }
    }
}

use opensearch::{http::transport::Transport, OpenSearch};

pub struct ServerState {
    pub client: OpenSearch,
}

impl ServerState {
    pub fn new(url: &str) -> ServerState {
        let transport = Transport::single_node(url).unwrap();
        let client = OpenSearch::new(transport);

        ServerState { client: client }
    }
}

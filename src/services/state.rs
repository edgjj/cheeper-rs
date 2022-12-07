use opensearch::{
    auth::Credentials,
    http::{
        transport::{SingleNodeConnectionPool, TransportBuilder},
        Url,
    },
    OpenSearch,
};

#[derive(Clone)]
pub struct ServerState {
    pub client: OpenSearch,
}

impl ServerState {
    pub fn new(url: String, username: String, password: String) -> ServerState {
        // we do a ton of unwraps since this is initial stage of app
        let parsed_url = Url::parse(url.as_str()).unwrap();

        let conn_pool = SingleNodeConnectionPool::new(parsed_url);
        let transport = TransportBuilder::new(conn_pool)
            .auth(Credentials::Basic(username.clone(), password.clone()))
            .build()
            .unwrap();

        let client = OpenSearch::new(transport);

        ServerState { client: client }
    }
}

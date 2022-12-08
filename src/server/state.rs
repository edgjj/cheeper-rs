use opensearch::{
    auth::Credentials,
    http::{
        transport::{SingleNodeConnectionPool, TransportBuilder},
        Url,
    },
    OpenSearch,
};

#[derive(Clone)]
pub struct State {
    pub client: OpenSearch,
}

impl State {
    #[must_use]
    pub fn new(url: String, username: String, password: String) -> Self {
        // we do a ton of unwraps since this is initial stage of app
        let parsed_url = Url::parse(url.as_str()).unwrap();

        let conn_pool = SingleNodeConnectionPool::new(parsed_url);
        let transport = TransportBuilder::new(conn_pool)
            .auth(Credentials::Basic(username, password))
            .build()
            .unwrap();

        let client = OpenSearch::new(transport);

        Self { client }
    }
}

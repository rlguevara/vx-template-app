use lazy_static::lazy_static;

lazy_static! {
    pub static ref AUTH_GRAOHQL_ENDPOINT: String = "https://test/v1/graphql".into();
    pub static ref AUTH_GRAOHQL_WS_ENDPOINT: String = "wss://test/v1/graphql".into();
    pub static ref AUTH_X_HASURA_ACCESS_KEY: String = "test".into();
}

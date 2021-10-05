use lazy_static::lazy_static;

lazy_static! {
    pub static ref AUTH_GRAOHQL_ENDPOINT: String = "https://api.vx-template-app.network/v1/graphql".into();
    pub static ref AUTH_GRAOHQL_WS_ENDPOINT: String = "wss://api.vx-template-app.network/v1/graphql".into();
    pub static ref AUTH_X_HASURA_ACCESS_KEY: String = "vx-template-app-access-key".into();
}

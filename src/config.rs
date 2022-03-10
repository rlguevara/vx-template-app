use dotenv_codegen::dotenv;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref AUTH_GRAOHQL_ENDPOINT: String = dotenv!("HASURA_ENDPOINT").to_string();
    pub static ref AUTH_GRAOHQL_WS_ENDPOINT: String = dotenv!("HASURA_WS_ENDPOINT").to_string();
    pub static ref AUTH_X_HASURA_ACCESS_KEY: String =
        dotenv!("HASURA_GRAPHQL_ADMIN_SECRET").to_string();
}

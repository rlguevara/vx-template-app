use crate::hasura;
use graphql_client::GraphQLQuery;
use chrono::prelude::*;

pub type Timestamptz = String;

pub fn wasm_utc_now() -> DateTime<chrono::Utc> {
    let timestamp = js_sys::Date::new_0().get_time();
    let secs = timestamp.floor();
    let nanoes = (timestamp - secs) * 1_000_000_000f64;
    let naivetime = NaiveDateTime::from_timestamp(secs as i64, nanoes as u32);
    DateTime::from_utc(naivetime, Utc)
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct TimeAdd;
impl hasura::Request for TimeAdd {}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct OnTimeAdded;
impl hasura::Subscribe for OnTimeAdded {}

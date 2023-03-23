use crate::graphql;
use graphql_client::GraphQLQuery;

// Show category data
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../../schema.graphql",
    query_path = "../../query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct ShowCategory;
impl graphql::Request for ShowCategory{}

// Show category data by id
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../../schema.graphql",
    query_path = "../../query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct ShowCategoryById;
impl graphql::Request for ShowCategoryById{}

// Add category data
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../../schema.graphql",
    query_path = "../../query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct AddCategory;
impl graphql::Request for AddCategory{}
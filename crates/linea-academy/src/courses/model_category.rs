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
pub struct CategoryData;
impl graphql::Request for CategoryData{}

// Add new category
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../../schema.graphql",
    query_path = "../../query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct AddCategory;
impl graphql::Request for AddCategory{}
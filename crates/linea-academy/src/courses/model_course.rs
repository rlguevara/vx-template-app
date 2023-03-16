use crate::graphql;
use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../../schema.graphql",
    query_path = "../../query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct CourseByCategory;
impl graphql::Request for CourseByCategory{}


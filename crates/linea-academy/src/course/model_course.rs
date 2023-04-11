use crate::graphql;
use graphql_client::GraphQLQuery;

// Show course data by category
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../../schema.graphql",
    query_path = "../../query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct ShowCoursesByCategory;
impl graphql::Request for ShowCoursesByCategory{}

// Show course data by id
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../../schema.graphql",
    query_path = "../../query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct ShowCourseById;
impl graphql::Request for ShowCourseById{}

// Add course data
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../../schema.graphql",
    query_path = "../../query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct AddCourse;
impl graphql::Request for AddCourse{}

// Delete course data
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../../schema.graphql",
    query_path = "../../query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct DeleteCourse;
impl graphql::Request for DeleteCourse{}

// Show content data by course
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../../schema.graphql",
    query_path = "../../query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct ShowContentByCourse;
impl graphql::Request for ShowContentByCourse{}

// Show content data by id
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "../../schema.graphql",
    query_path = "../../query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct ShowContentById;
impl graphql::Request for ShowContentById{}
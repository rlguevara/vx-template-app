use crate::auth::*;
use graphql_client::{GraphQLQuery, QueryBody};
use log::*;
use yew::format::Json;
use yew::services::fetch::{self, FetchService, FetchTask, Response};
use yew::{Component, ComponentLink};

fn query_req<T>(query: Json<T>) -> fetch::Request<Json<T>> {
    fetch::Request::post(AUTH_GRAOHQL_ENDPOINT.to_string())
        .header("Content-Type", "application/json")
        .header("x-hasura-access-key", AUTH_X_HASURA_ACCESS_KEY.to_string())
        .body(query)
        .expect("Failed to build request.")
}

pub trait Request {
    fn request<C, M, F>(
        link: &ComponentLink<C>,
        vars: Self::Variables,
        response: F,
    ) -> Result<FetchTask, anyhow::Error>
    where
        Self: GraphQLQuery + 'static,
        C: Component,
        M: Into<C::Message>,
        F: Fn(Option<Self::ResponseData>) -> M + 'static,
    {
        let query: QueryBody<Self::Variables> = Self::build_query(vars);
        let post_request = query_req(Json(&query));
        let post_callback = link.callback(
            move |callback: Response<
                Json<Result<graphql_client::Response<Self::ResponseData>, anyhow::Error>>,
            >| {
                let (_meta, Json(result)) = callback.into_parts();
                match result {
                    Ok(gq_response) => {
                        if let Some(errors) = gq_response.errors {
                            error!("Error handling request: {:?}", errors);
                            response(None)
                        } else {
                            response(gq_response.data)
                        }
                    }
                    Err(error) => {
                        error!("Error handling request: {:?}", &error);
                        response(None)
                    }
                }
            },
        );
        FetchService::fetch(post_request, post_callback)
    }
}

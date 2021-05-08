use graphql_client::{GraphQLQuery, QueryBody};
use log::*;
use serde::ser::Serialize;
use serde_json;
use wasm_bindgen::prelude::*;
use yew::{Callback, Component, ComponentLink};

pub struct Client(JsValue);

pub struct Subscription(JsValue, Closure<dyn Fn(String)>);

impl Drop for Subscription {
    fn drop(&mut self) {
        debug!("Subscription dropped");
        unsafe { unsubscribe(self) }
    }
}

pub struct SubscriptionService {
    client: Client,
}

impl std::fmt::Debug for SubscriptionService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SubscriptionService")
    }
}

impl SubscriptionService {
    pub fn new() -> Self {
        SubscriptionService {
            client: unsafe { init_client() },
        }
    }
}

pub trait Subscribe {
    fn subscribe<C, M, F>(
        service: &SubscriptionService,
        link: &ComponentLink<C>,
        vars: Self::Variables,
        on_response: F,
    ) -> Subscription
    where
        Self: GraphQLQuery,
        C: Component,
        M: Into<C::Message>,
        F: Fn(Self::ResponseData) -> M + 'static,
    {
        let query: QueryBody<Self::Variables> = Self::build_query(vars);
        let callback: Callback<String> = link.callback(move |json: String| {
            let data: Result<Self::ResponseData, serde_json::Error> = serde_json::from_str(&json);
            on_response(data.unwrap())
        });
        unsafe { subscribe(&service.client, &query, callback) }
    }
}

#[wasm_bindgen(module = "/src/hasura/subscription.js")]
extern "C" {
    #[wasm_bindgen(js_name = "initClient")]
    fn init_client_js() -> JsValue;

    #[wasm_bindgen(js_name = "subscribe")]
    unsafe fn subscribe_js(client: &JsValue, query: String, callback: &JsValue) -> JsValue;

    #[wasm_bindgen(js_name = "unsubscribe")]
    fn unsubscribe_js(subscription: &JsValue) -> JsValue;
}

unsafe fn init_client() -> Client {
    let client = init_client_js();
    Client(client)
}

unsafe fn subscribe<Q: Serialize>(
    client: &Client,
    query: &Q,
    callback: Callback<String>,
) -> Subscription {
    let Client(client) = client;
    let query = serde_json::to_string(&query).unwrap();
    let closure =
        Closure::wrap(Box::new(move |data: String| callback.emit(data)) as Box<dyn Fn(String)>);
    let sub = subscribe_js(client, query, closure.as_ref());
    Subscription(sub, closure)
}

unsafe fn unsubscribe(subscription: &Subscription) {
    let Subscription(subscription, _closure) = subscription;
    unsubscribe_js(&subscription);
}

use super::protocol::*;
use crate::auth::*;
use graphql_client::{GraphQLQuery, QueryBody};
use log::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CloseEvent, ErrorEvent, MessageEvent, WebSocket};
use yew::{Callback, Component, ComponentLink};

type ServiceStateHandle = Rc<RefCell<ServiceState>>;
type SubscriptionStateHandle = Rc<SubscriptionState>;

struct SubscriptionState {
    uuid: Uuid,
    payload: ClientMessage,
    callback: Callback<String>,
    service: ServiceStateHandle,
}

pub struct Subscription {
    handle: SubscriptionStateHandle,
}

impl Drop for Subscription {
    fn drop(&mut self) {
        info!("SubscriptionService: Dropped");
        let id = self.handle.uuid.to_string();
        let msg = ClientMessage::Complete { id: id.clone() };
        let msg = serde_json::to_string(&msg).unwrap();
        let mut service = self.handle.service.borrow_mut();
        service.subs.remove(&id);
        match &service.ws {
            Some(ws) => {
                let res = ws.send_with_str(&msg);
                if let Err(err) = res {
                    error!("SubscriptionService: {:?}", err);
                }
            }
            None => {
                error!("SubscriptionService: Invalid WebSocket");
            }
        }
    }
}

struct ServiceState {
    ws: Option<WebSocket>,
    subs: HashMap<String, Weak<SubscriptionState>>,
}

pub struct SubscriptionService {
    handle: ServiceStateHandle,
}

impl std::fmt::Debug for SubscriptionService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SubscriptionService")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Headers {
    #[serde(rename = "headers")]
    Auth {
        #[serde(rename = "x-hasura-admin-secret")]
        #[serde(skip_serializing_if = "Option::is_none")]
        x_hasura_admin_secret: Option<String>,
    },
}

impl SubscriptionService {
    pub fn new() -> SubscriptionService {
        let ws = WebSocket::new_with_str(&AUTH_GRAOHQL_WS_ENDPOINT, "graphql-transport-ws").ok();

        let service: ServiceStateHandle = Rc::new(RefCell::new(ServiceState {
            ws: ws.clone(),
            subs: HashMap::new(),
        }));

        if let Some(ws) = ws.clone() {
            // OnMessage
            let _cloned_ws = ws.clone();
            let cloned_service = service.clone();
            let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
                if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                    warn!(
                        "SubscriptionService: OnMessage, received arraybuffer: {:?}",
                        abuf
                    );
                } else if let Ok(blob) = e.data().dyn_into::<web_sys::Blob>() {
                    warn!("SubscriptionService: OnMessage, received blob: {:?}", blob);
                } else if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                    debug!("SubscriptionService: OnMessage, received Text: {:?}", txt);
                    let msg = serde_json::from_str::<ServerMessage>(
                        txt.as_string().unwrap_or_default().as_str(),
                    );
                    match msg {
                        Ok(msg) => match msg {
                            ServerMessage::ConnectionAck => {
                                info!("SubscriptionService: ConnectionAck [Handled]");
                            }
                            ServerMessage::Ping => {
                                debug!("SubscriptionService: Ping [Handled]");
                                // let payload = ClientMessage::Pong {};
                                // let payload = serde_json::to_string(&payload).unwrap_or_default();
                                // let res = cloned_ws.send_with_str(&payload);
                                // if let Err(err) = res {
                                //     error!("SubscriptionService: {:?}", err);
                                // }
                            }
                            ServerMessage::Next { id, payload } => {
                                info!("SubscriptionService: Next [Handled] {:?} {:?}", id, payload);
                                let service = cloned_service.borrow();
                                if let Some(sub) = service.subs.get(&id) {
                                    if let Some(sub) = sub.upgrade() {
                                        if let Some(data) = payload.data {
                                            sub.callback.emit(data.to_string());
                                        }
                                    }
                                }
                            }
                            _ => {
                                warn!("SubscriptionService: {:?} [NOT Handled]", msg);
                            }
                        },
                        Err(err) => {
                            error!("SubscriptionService: {:?}", err)
                        }
                    };
                } else {
                    warn!(
                        "SubscriptionService: OnMessage, received Unknown: {:?}",
                        e.data()
                    );
                }
            }) as Box<dyn FnMut(MessageEvent)>);
            ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
            onmessage_callback.forget();

            // OnError
            let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
                error!("SubscriptionService: OnError: {:?}", e);
            }) as Box<dyn FnMut(ErrorEvent)>);
            ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
            onerror_callback.forget();

            // OnClose
            let onclose_callback = Closure::wrap(Box::new(move |e: CloseEvent| {
                info!("SubscriptionService: OnClose: {:?}", e);
            }) as Box<dyn FnMut(CloseEvent)>);
            ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
            onclose_callback.forget();

            // OnOpen
            let cloned_ws = ws.clone();
            let onopen_callback = Closure::wrap(Box::new(move |_| {
                info!("SubscriptionService: OnOpen");
                let msg = Headers::Auth {
                    x_hasura_admin_secret: Some("l1n3ar0sa4-4p1".to_string()),
                };
                let headers = serde_json::to_value(&msg).ok();
                let payload = ClientMessage::ConnectionInit { payload: headers };
                let payload = serde_json::to_string(&payload).unwrap_or_default();
                info!("SubscriptionService: {:?}", payload);
                let res = cloned_ws.send_with_str(&payload);
                info!("SubscriptionService: {:?}", res);
            }) as Box<dyn FnMut(JsValue)>);
            ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
            onopen_callback.forget();
        }

        SubscriptionService { handle: service }
    }

    fn restore(&self, state: &SubscriptionState) {
        let msg_str = serde_json::to_string(&state.payload);
        match msg_str {
            Ok(msg_str) => {
                info!("SubscriptionService: Subscribe: {:?}", msg_str);
                if let Some(ws) = &self.handle.borrow().ws {
                    let res = ws.send_with_str(&msg_str);
                    if let Err(err) = res {
                        error!("SubscriptionService: {:?}", err);
                    }
                }
            }
            Err(err) => {
                error!("SubscriptionService: {:?}", err);
            }
        };
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
        F: Fn(Option<Self::ResponseData>) -> M + 'static,
    {
        let uuid = Uuid::new_v4();
        let vars_value = serde_json::to_value(&vars).ok();
        let query: QueryBody<Self::Variables> = Self::build_query(vars);
        let callback: Callback<String> = link.callback(move |json: String| {
            let data: Result<Self::ResponseData, serde_json::Error> = serde_json::from_str(&json);
            on_response(data.ok())
        });
        let payload = ClientMessage::Subscribe {
            id: uuid.to_string(),
            payload: ClientPayload {
                variables: vars_value,
                query: query.query.to_string(),
                operation_name: Some(query.operation_name.to_string()),
            },
        };
        let sub = SubscriptionState {
            uuid,
            payload,
            service: service.handle.clone(),
            callback,
        };
        let sub: SubscriptionStateHandle = Rc::new(sub);
        {
            let mut service_state = service.handle.borrow_mut();
            service_state
                .subs
                .insert(uuid.to_string(), Rc::downgrade(&sub));
        }
        service.restore(&sub);
        Subscription { handle: sub }
    }
}

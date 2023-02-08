use super::model::*;
use crate::graphql::{
    GraphQLService, GraphQLTask, Request, RequestTask, Subscribe, SubscriptionTask,
};
use code_location::code_location;
use log::*;
use yew::prelude::*;

#[derive(Debug)]
pub enum TestViewMessage {
    TimeAdd,
    TimeAdded(Option<time_add::ResponseData>),
    OnTimeAdded(Option<on_time_added::ResponseData>),
    SubscriptionEnable,
    SubscriptionDisable,
    Subscribe,
    Unsubscribe,
}

pub struct TestView {
    graphql_task: Option<GraphQLTask>,
    req_task: Option<RequestTask>,
    sub_task: Option<SubscriptionTask>,
}

impl Component for TestView {
    type Message = TestViewMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            graphql_task: None,
            req_task: None,
            sub_task: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        // info!("{:?}", msg);
        match msg {
            Self::Message::TimeAdd => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = time_add::Variables {
                        time: wasm_utc_now().to_rfc3339(),
                    };
                    let task = TimeAdd::request(graphql_task, &ctx, vars, |data| {
                        Self::Message::TimeAdded(data)
                    });
                    self.req_task = Some(task);
                }
            }
            Self::Message::TimeAdded(data) => {
                info!("TimeAdded: {:?}", data)
            }
            Self::Message::OnTimeAdded(data) => {
                info!("OnTimeAdded: {:?}", data)
            }
            Self::Message::SubscriptionEnable => {
                self.graphql_task = Some(GraphQLService::connect(&code_location!()))
            }
            Self::Message::SubscriptionDisable => self.graphql_task = None,
            Self::Message::Subscribe => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = on_time_added::Variables {};
                    let sub_task = OnTimeAdded::subscribe(graphql_task, &ctx, vars, |data| {
                        Self::Message::OnTimeAdded(data)
                    });
                    self.sub_task = Some(sub_task);
                }
            }
            Self::Message::Unsubscribe => {
                self.sub_task = None;
            }
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        // info!("{:?}", props);
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let graphql_task = if self.graphql_task.is_some() {
            html! { <button class="button is-dark my-1" onclick={ctx.link().callback(|_| Self::Message::SubscriptionDisable)}>{"Subscription Disable"}</button> }
        } else {
            html! { <button class="button is-dark my-1" onclick={ctx.link().callback(|_| Self::Message::SubscriptionEnable)}>{"Subscription Enable"}</button> }
        };

        let subscribed = if self.sub_task.is_some() {
            html! { <button class="button is-dark my-1" onclick={ctx.link().callback(|_| Self::Message::Unsubscribe)}>{"Unsubscribe"}</button> }
        } else {
            html! { <button class="button is-dark my-1" onclick={ctx.link().callback(|_| Self::Message::Subscribe)}>{"Subscribe"}</button> }
        };
        let position_add = ctx.link().callback(|_| Self::Message::TimeAdd);
        html! {
            <>
                <button class="button is-dark my-1" onclick={position_add}>{"Send Location"}</button>
                <br />
                { graphql_task }
                <br />
                { subscribed }
            </>
        }
    }
}

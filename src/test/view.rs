use super::model::*;
use crate::hasura::{Request, Subscribe, Subscription, SubscriptionService};
use log::*;
use yew::{prelude::*, services::fetch::FetchTask};

#[derive(Debug)]
pub enum TestViewMessage {
    TimeAdd,
    TimeAdded(Option<time_add::ResponseData>),
    OnTimeAdded(Option<on_time_added::ResponseData>),
    Subscribe,
    Unsubscribe,
}

pub struct TestView {
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
    sub_service: SubscriptionService,
    sub: Option<Subscription>,
}

impl Component for TestView {
    type Message = TestViewMessage;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            task: None,
            sub_service: SubscriptionService::new(),
            sub: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        match msg {
            Self::Message::TimeAdd => {
                let vars = time_add::Variables {
                    time: wasm_utc_now().to_rfc3339(),
                };
                let task =
                    TimeAdd::request(&self.link, vars, |data| Self::Message::TimeAdded(data));
                self.task = task.ok();
            }
            Self::Message::TimeAdded(data) => {
                info!("TimeAdded: {:?}", data)
            }
            Self::Message::OnTimeAdded(data) => {
                info!("OnTimeAdded: {:?}", data)
            }
            Self::Message::Subscribe => {
                let vars = on_time_added::Variables {};
                let sub = OnTimeAdded::subscribe(&self.sub_service, &self.link, vars, |data| {
                    Self::Message::OnTimeAdded(data)
                });
                self.sub = Some(sub);
            }
            Self::Message::Unsubscribe => {
                self.sub = None;
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("{:?}", props);
        true
    }

    fn view(&self) -> Html {
        let subscribed = if self.sub.is_some() {
            html! { <button onclick=self.link.callback(|_| Self::Message::Unsubscribe)>{"Unsubscribe"}</button> }
        } else {
            html! { <button onclick=self.link.callback(|_| Self::Message::Subscribe)>{"Subscribe"}</button> }
        };
        let position_add = self.link.callback(|_| Self::Message::TimeAdd);
        html! {
            <>
                <button onclick=position_add>{"Send Location"}</button>
                { subscribed }
            </>
        }
    }
}

use super::model_course::*;
use yew::prelude::*;

use crate::graphql::{
    GraphQLService, GraphQLTask, Request, RequestTask};
use code_location::code_location;

pub struct ContentById {
    graphql_task: Option<GraphQLTask>,
    req_task: Option<RequestTask>,
    content: Option<show_content_by_id::ResponseData>,
    content_id: i64,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub content_id: i64,
}

pub enum ContentByIdMessage {
    ShowContentById,
    ContentReceived(Option<show_content_by_id::ResponseData>)
}

impl Component for ContentById {
    type Message = ContentByIdMessage;
    type Properties = Props;

    fn create (ctx: &Context<Self>) -> Self {
        // let show_data = ctx.link().send_message(Self::Message::ShowCourseData);
        Self {
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            req_task: None,
            content: None,
            content_id: ctx.props().content_id,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::ShowContentById => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = show_content_by_id::Variables {
                        eq: Some(ctx.props().content_id),
                    };
                    let task = ShowContentById::request(graphql_task, &ctx, vars, |data| {
                        Self::Message::ContentReceived(data)
                    });
                    self.req_task = Some(task);
                }
            }
            Self::Message::ContentReceived(data) => {
                self.content = data.clone();
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let show_content_data = ctx.link().callback(|_| Self::Message::ShowContentById);
        let list_content = self.content.clone().and_then(|data| Some(data.lr_academy_course_content)).unwrap_or_default().iter().map(|content| {
            html!{
                <div>
                    <span>{&content.content_name}</span>
                    <span>{&content.content_description}</span>
                </div>
            }
            }).collect::<Html>();
        html! {
            <div>
                <h1>{"HELLO?"}</h1>
            <button class="button is-dark my-1" onclick={show_content_data}>{"Show content"}</button>
            {list_content}
            </div>
        }
    }

}
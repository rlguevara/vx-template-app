use super::model_course::*;
use yew::prelude::*;
use yew_router::{prelude::*, navigator};
use log::*;

use crate::graphql::{
    GraphQLService, GraphQLTask, Request, RequestTask};
use code_location::code_location;
use crate::components::router::Route;

pub struct ShowContentByCourseList {
    graphql_task: Option<GraphQLTask>,
    req_task: Option<RequestTask>,
    content: Option<show_content_by_course::ResponseData>,
    content_id: i64,
}

pub enum ShowContentByCourseListMessage {
    ShowContentData,
    ContentReceived(Option<show_content_by_course::ResponseData>)
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub content_id: i64,
}


impl Component for ShowContentByCourseList {
    type Message = ShowContentByCourseListMessage;
    type Properties = Props;

    fn create (ctx: &Context<Self>) -> Self {
        let show_data = ctx.link().send_message(Self::Message::ShowContentData);
        Self {
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            req_task: None,
            content: None,
            content_id: ctx.props().content_id,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::ShowContentData => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = show_content_by_course::Variables {
                        eq: Some(ctx.props().content_id),
                    };
                    let task = ShowContentByCourse::request(graphql_task, &ctx, vars, |data| {
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
        let content = self.content.clone();
        let navigator = Some(ctx.link().navigator()).unwrap();
        let list_content = self.content.clone().and_then(|data| Some(data.lr_academy_course_content)).unwrap_or_default().iter().map(|content| {
            let content_id_data = content.id.clone();
            let navigator = navigator.clone().unwrap();
            let onclick = Callback::from(move |_| navigator.push(&Route::Courses { id: content_id_data}));
            html!{
                <div>
                    <div>{&content.content_name}</div>
                    <div>{&content.content_description}</div>
                    <div>{&content.content_url}</div>
                    <div>{content.id}</div>
                    <div>{"Hello?"}</div>
                    <button class="button is-link" onclick={onclick}>{"Read more..."}</button>
                </div>
            }
            }).collect::<Html>();
        html! {
            <div>
            {list_content}
            </div>
        }
    }

}

use super::model_course::*;
use yew::prelude::*;

use crate::graphql::{
    GraphQLService, GraphQLTask, Request, RequestTask};
use code_location::code_location;

pub struct CourseById {
    graphql_task: Option<GraphQLTask>,
    req_task: Option<RequestTask>,
    course: Option<show_course_by_id::ResponseData>,
    course_id: i64,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub course_id: i64,
}

pub enum CourseByIdMessage {
    ShowCourseById,
    CourseReceived(Option<show_course_by_id::ResponseData>)
}

impl Component for CourseById {
    type Message = CourseByIdMessage;
    type Properties = Props;

    fn create (ctx: &Context<Self>) -> Self {
        // let show_data = ctx.link().send_message(Self::Message::ShowCourseData);
        Self {
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            req_task: None,
            course: None,
            course_id: ctx.props().course_id,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::ShowCourseById => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = show_course_by_id::Variables {
                        eq: Some(ctx.props().course_id),
                    };
                    let task = ShowCourseById::request(graphql_task, &ctx, vars, |data| {
                        Self::Message::CourseReceived(data)
                    });
                    self.req_task = Some(task);
                }
            }
            Self::Message::CourseReceived(data) => {
                self.course = data.clone();
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let show_course_data = ctx.link().callback(|_| Self::Message::ShowCourseById);
        let list_course = self.course.clone().and_then(|data| Some(data.lr_academy_course)).unwrap_or_default().iter().map(|course| {
            html!{
                <div>
                    <span>{&course.course_name}</span>
                    <span>{&course.course_description}</span>
                </div>
            }
            }).collect::<Html>();
        html! {
            <div>
                <h1>{"HELLO?"}</h1>
            <button class="button is-dark my-1" onclick={show_course_data}>{"Show course"}</button>
            {list_course}
            </div>
        }
    }

}
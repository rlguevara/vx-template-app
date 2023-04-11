use super::model_course::*;
use yew::prelude::*;

use crate::graphql::{
    GraphQLService, GraphQLTask, Request, RequestTask};
use code_location::code_location;
use log::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub course_id: i64
}

pub struct DelCourse {
    graphql_task: Option<GraphQLTask>,
    insert_task: Option<RequestTask>,
    course_id: i64,
}

pub enum DelCourseCourseMessage {
    DeleteCourse,
    CourseDeleted(Option<delete_course::ResponseData>)
}

impl Component for DelCourse {
    type Message = DelCourseCourseMessage;
    type Properties = Props;

    fn create (ctx: &Context<Self>) -> Self {
        Self {
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            insert_task: None,
            course_id: ctx.props().course_id,
        }
    }

    fn update (&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::DeleteCourse => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = delete_course::Variables {
                        id: Some(self.course_id.clone()),
                    };
                    let task = DeleteCourse::request(graphql_task, &ctx, vars, |data| {
                        Self::Message::CourseDeleted(data)
                    });
                    self.insert_task = Some(task);
                }
            }
            Self::Message::CourseDeleted(data) => {
                info!("CourseDeleted: {:?}", data);
            }
        }
        true
    }

    fn view (&self, ctx: &Context<Self>) -> Html {
        let show_id = ctx.props().course_id;
        html! {
            <>
            <a href="#" class="btn btn-danger m-1" onclick={ctx.link().callback(|_| Self::Message::DeleteCourse)}><i class="fa-solid fa-trash"></i></a>
            </>
        }
    }
}
use super::model_course::*;
use yew::prelude::*;

use crate::graphql::{
    GraphQLService, GraphQLTask, Request, RequestTask};
use code_location::code_location;
use crate::components::courseitem::CourseItem;

pub struct ShowCourseList {
    graphql_task: Option<GraphQLTask>,
    req_task: Option<RequestTask>,
    course: Option<show_courses_by_category::ResponseData>,
    course_category_id: i64,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub category_id: i64,
}

pub enum ShowCoursesMessage {
    ShowCourseData,
    CourseReceived(Option<show_courses_by_category::ResponseData>)
}

impl Component for ShowCourseList {
    type Message = ShowCoursesMessage;
    type Properties = Props;

    fn create (ctx: &Context<Self>) -> Self {
        let show_data = ctx.link().send_message(Self::Message::ShowCourseData);
        Self {
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            req_task: None,
            course: None,
            course_category_id: ctx.props().category_id,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::ShowCourseData => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = show_courses_by_category::Variables {
                        eq: Some(ctx.props().category_id),
                    };
                    let task = ShowCoursesByCategory::request(graphql_task, &ctx, vars, |data| {
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
        // let show_courses_data = ctx.link().callback(|_| Self::Message::ShowCourseData);

        // let list_category = self.category.clone().and_then(|data| Some(data.category)).unwrap_or_default().iter().map(|category| {
        //     html!{
        //         <CategoryItem title={category.name.clone()} description={category.description.clone()} />
        //     }
        //     }).collect::<Html>();
        let list_courses = self.course.clone().and_then(|data| Some(data.lr_academy_course)).unwrap_or_default().iter().map(|course| {
            html!{
                <CourseItem course_id={course.id} course_name={course.course_name.clone()}/>
                // <div>
                //     <span>{&course.course_name}</span>
                //     <span>{&course.course_description}</span>
                //     <span>{course.category_id}</span>
                // </div>
            }
            }).collect::<Html>();
        html! {
            <div>

            // <button class="button is-dark my-1" onclick={show_courses_data}>{"Show courses"}</button>
            {list_courses}
            </div>
        }
    }

}
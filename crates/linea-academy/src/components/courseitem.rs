use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::router::Route;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub course_id: i64,
    pub course_name: String,
}

#[function_component(CourseItem)]
pub fn course_item(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let course_id = props.course_id;
    let onclick = Callback::from(move |_| navigator.push(&Route::Content { id: course_id}));
    html! {
        <div class="col-lg-4 col-md-6 align-self-center mb-30 col-md-6">
            <div class="events_item">
                <div class="thumb">
                    <a onclick={onclick}><img src="assets/images/course-01.jpg" alt=""/></a>
                    // <span class="category">{"Cultura"}</span>
                    <span class="price"><h6><em>{"$"}</em>{"160"}</h6></span>
                </div>
                <div class="down-content">
                    <span class="author">{"Stella Blair"}</span>
                    <h4>{&props.course_name}</h4>
                </div>
            </div>
        </div>
    }
}
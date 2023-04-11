use yew::prelude::*;
use crate::course::view_show_content_by_course::ShowContentByCourseList;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub course_id: i64,
}

#[function_component(Content)]
pub fn content(props: &Props) -> Html {
    html! {
        <div class="section events" id="events">
        <div class="container">
          <div class="row">
            <div class="col-lg-12 text-center">
              <div class="section-heading">
                <h6>{"Schedule"}</h6>
                <h2>{"Upcoming Events"}</h2>
              </div>
            </div>
            <ShowContentByCourseList course_id={props.course_id}/>
          </div>
        </div>
      </div>
    }
}
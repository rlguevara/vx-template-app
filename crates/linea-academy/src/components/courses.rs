use yew::prelude::*;
use crate::course::view_show_courses::ShowCourseList;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub category_id: i64,
}

#[function_component(CoursesList)]
pub fn courses_list(props: &Props) -> Html {
    html! {
        <section class="section courses" id="courses" >
        <div class="container">
            <div class="row">
                <div class="col-lg-12 text-center">
                <div class="section-heading">
                    <h6>{"Cursos agregados"}</h6>
                    <h2>{"Cursos Agregados"}</h2>
                </div>
                </div>
            </div>
            <div class="row">
                <ShowCourseList category_id={props.category_id}/>
            </div>
        </div>
    </section>
    }
}
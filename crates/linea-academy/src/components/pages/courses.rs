use yew::prelude::*;
use crate::components::{
    header::Header,
    courses::CoursesList,
    footer::Footer,
};
use crate::category::view_show_category_by_id::CategoryById;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub category_id: i64,
}

#[function_component(Courses)]
pub fn courses(props: &Props) -> Html {
    html! {
        <>
            <Header />
            <CategoryById category_id={props.category_id}/>
            <CoursesList category_id={props.category_id}/>
            <Footer />
        </>
    }
}
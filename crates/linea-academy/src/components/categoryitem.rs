use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::router::Route;

#[derive(Properties, Clone, PartialEq )]
pub struct Props {
    pub title: String,
    pub description: String,
    pub category_id: i64,
}

#[function_component(CategoryItem)]
pub fn categoryitem(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let category_id = props.category_id;
    let onclick = Callback::from(move |_| navigator.push(&Route::Courses { id: category_id}));
    html! {
        <div class="col-lg-4 col-md-6">
            <div class="service-item">
                <div class="icon">
                    <img src="assets/images/service-01.png" alt="online degrees"/>
                </div>
                <div class="main-content">
                    <h4>{&props.title}</h4>
                    <p>{&props.description}</p>
                    <div class="main-button">
                        <button onclick={onclick}>{"Saber más..."}</button>
                    </div>
                </div>
            </div>
        </div>
    }
}
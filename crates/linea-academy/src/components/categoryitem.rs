use yew::prelude::*;

#[derive(Properties, Clone, PartialEq )]
pub struct Props {
    pub title: String,
    pub description: String,
}

#[function_component(CategoryItem)]
pub fn categoryitem(props: &Props) -> Html {
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
                        <a href="#">{"Read More"}</a>
                    </div>
                </div>
            </div>
        </div>
    }
}
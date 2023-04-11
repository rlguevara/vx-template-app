use yew::prelude::*;
use crate::category::view_delete_category::DelCategory;
use crate::category::view_update_category::UpdtCategory;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub category_name: String,
    pub category_description: String,
    pub category_image: String,
    pub category_id: i64,
}

#[function_component(CategoryInfo)]
pub fn category_info(props: &Props) -> Html {
    html! {
        <div class="main-banner" id="top">
            <div class="container">
                <div class="row">
                    <div class="col-lg-12">
                        // <div class="owl-carousel owl-banner">
                            <div class="item item-1">
                                <div class="header-text">
                                    <span class="category">{"Nuestras certificaciones"}</span>
                                    <h2>{&props.category_name}</h2>
                                    <p>{&props.category_description}</p>
                                    <div class="buttons">
                                        <UpdtCategory category_id={props.category_id}
                                                        category_name={props.category_name.clone()}
                                                        category_description={props.category_description.clone()}
                                                        category_image={props.category_image.clone()}
                                                        />
                                        <DelCategory category_id={props.category_id} />
                                    </div>
                                </div>
                            </div>
                        // </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
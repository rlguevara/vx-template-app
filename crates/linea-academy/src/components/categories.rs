use yew::prelude::*;
use crate::courses::view_show_category::ShowCategory;

// #[derive(Clone, PartialEq, Properties)]
// pub struct Props {
//     pub title: String,
//     pub description: String,
// }

#[function_component(Categories)]
pub fn categories() -> Html {
    html! {
      <div class="services section" id="services">
        <div class="container">
          <div class="row">
                <ShowCategory />
          </div>
        </div>
      </div>
    }
}
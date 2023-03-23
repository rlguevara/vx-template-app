use yew::prelude::*;
use crate::category::view_show_category::ShowCategoryList;

#[function_component(Categories)]
pub fn categories() -> Html {
    html! {
      <div class="services section" id="services">
        <div class="container">
          <div class="row">
                <ShowCategoryList />
          </div>
        </div>
      </div>
    }
}
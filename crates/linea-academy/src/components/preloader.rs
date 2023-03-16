use yew::prelude::*;

#[function_component(Preloader)]
pub fn preloader() -> Html {
    html! {
        <div id="js-preloader" class="js-preloader">
        <div class="preloader-inner">
          <span class="dot"></span>
          <div class="dots">
            <span></span>
            <span></span>
            <span></span>
          </div>
        </div>
      </div>
    }
}
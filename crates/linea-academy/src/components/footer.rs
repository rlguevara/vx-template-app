use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
            <div class="container">
                <div class="col-lg-12">
                    <p>{"Copyright © 2036 Scholar Organization. All rights reserved. Design:"} <a href="https://templatemo.com" rel="nofollow" target="_blank">{"TemplateMo"}</a></p>
                </div>
            </div>
        </footer>
    }
}
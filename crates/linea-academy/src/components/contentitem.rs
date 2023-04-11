use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::router::Route;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub course_id: i64,
    pub content_id: i64,
    pub content_name: String,
}

#[function_component(ContentItem)]
pub fn content_item(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let content_id = props.content_id;
    let onclick = Callback::from(move |_| navigator.push(&Route::ContentDetail { id: content_id}));
    html! {
        <>
        <div class="col-lg-12 col-md-6">
              <div class="item">
                <div class="row">
                    <div class="col-lg-9">
                        <ul>
                            <li>
                                <span class="category">{"Web Design"}</span>
                                <h4>{&props.content_name}</h4>
                            </li>
                            <li>
                                <span>{"Date:"}</span>
                                <h6>{"16 Feb 2036"}</h6>
                            </li>
                            <li>
                                <span>{"Duration:"}</span>
                                <h6>{"22 Hours"}</h6>
                            </li>
                            <li>
                                <span>{"Price:"}</span>
                                <h6>{"$120"}</h6>
                            </li>
                        </ul>
                        <a href="#" onclick={onclick}><i class="fa fa-angle-right"></i></a>
                    </div>
                </div>
              </div>
            </div>
        <div class="col-lg-3">
            <div class="image">
                <img src="assets/images/event-01.jpg" alt=""/>
            </div>
        </div>
       </> 
    }
}
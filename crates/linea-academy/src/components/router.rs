use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::pages::{home::Home, courses::Courses};

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/category/:id")]
    Courses {id: i64},
    #[at("/course/:id")]
    Content {id: i64},
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! {<Home />},
        Route::Courses {id}=> html! {<Courses category_id={id}/>},
        Route::Content {id}=> html! {{"Hello world"}},
    }
}
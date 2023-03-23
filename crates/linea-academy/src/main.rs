#![recursion_limit = "512"]
#![allow(dead_code)]
#![allow(unused_variables)]


extern crate lazy_static;

mod course;
mod category;
mod components;
pub mod graphql;

use log::*;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::router::{switch, Route};
mod config;

#[derive(Debug)]
enum Message {}

struct Model {}

impl Component for Model {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        info!("{:?}", ctx.props());
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <BrowserRouter>
                    <Switch<Route> render={switch}/>
                </BrowserRouter>

                <script src="../../assets/js/counter.js"></script>
                <script src="../../assets/js/custom.js"></script>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    yew::Renderer::<Model>::new().render();
}

#![recursion_limit = "512"]

extern crate lazy_static;

use log::*;
use test::*;
use yew::prelude::*;

pub mod hasura;
pub mod auth;
mod test;

#[derive(Debug)]
enum Message {}

struct Model {
    _link: ComponentLink<Self>,
}

impl Component for Model {
    type Message = Message;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { _link: link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("{:?}", props);
        true
    }

    fn view(&self) -> Html {
        html! {
            <TestView />
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    yew::start_app::<Model>();
}

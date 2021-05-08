#![recursion_limit = "512"]

use geojson::{Geometry, Value};
use hasura::{
    subscribe::{self},
    Request, Subscribe,
};
use log::*;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;
use yew::format::Json;
use yew::{
    prelude::*,
    services::{
        fetch::FetchTask,
        storage::{Area, StorageService},
        timeout, Task,
    },
};

mod hasura;

#[derive(Debug)]
enum Message {
}

struct Model {
    link: ComponentLink<Self>,
}

impl Component for Model {
    type Message = Message;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
        }
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
        html!{}
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    yew::start_app::<Model>();
}

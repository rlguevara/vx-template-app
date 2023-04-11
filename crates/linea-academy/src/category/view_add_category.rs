use super::model_category::*;
use yew::prelude::*;

use crate::graphql::{
    GraphQLService, GraphQLTask, Request, RequestTask};
use code_location::code_location;
use web_sys::Event;
use web_sys::HtmlInputElement;
use web_sys::InputEvent;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use log::*;

pub struct AddNewCategory {
    graphql_task: Option<GraphQLTask>,
    insert_task: Option<RequestTask>,
    category_id: i64,
    category_name: String,
    category_description: String,
    category_image_url: String,
}

pub enum AddNewCategoryMessage {
    InsertCategoryId(i64),
    InsertCategoryName(String),
    InsertCategoryDescription(String),
    InsertCategoryImageUrl(String),
    AddCategory,
    CategoryAdded(Option<add_category::ResponseData>),
}

impl AddNewCategory {
    pub fn get_value_from_input_event(e: InputEvent) -> String {
        let event: Event = e.dyn_into().unwrap_throw();
        let event_target = event.target().unwrap_throw();
        let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
        target.value()
    }
}

impl Component for AddNewCategory {
    type Message = AddNewCategoryMessage;
    type Properties = ();

    fn create (_ctx: &Context<Self>) -> Self {
        Self {
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            insert_task: None,
            category_id: 0,
            category_name: "".to_string(),
            category_description: "".to_string(),
            category_image_url: "".to_string()
        }
    }

    fn update (&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::InsertCategoryId(category_id) => {
                self.category_id = category_id;
            }
            Self::Message::InsertCategoryName(category_name) => {
                self.category_name = category_name;
            }
            Self::Message::InsertCategoryDescription(category_description) => {
                self.category_description = category_description;
            }
            Self::Message::InsertCategoryImageUrl(category_image_url) => {
                self.category_image_url = category_image_url;
            }
            Self::Message::AddCategory => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = add_category::Variables {
                        id: Some(self.category_id.clone()),
                        category_name: Some(self.category_name.clone()),
                        category_description: Some(self.category_description.clone()),
                        category_image_url: Some(self.category_image_url.clone())
                    };
                    let task = AddCategory::request(graphql_task, &ctx, vars, |data| {
                        Self::Message::CategoryAdded(data)
                    });
                    self.insert_task = Some(task);
                }
            }
            Self::Message::CategoryAdded(data) => {
                info!("CategoryAdded: {:?}", data);
            }
        }
        true
    }

    fn view (&self, ctx: &Context<Self>) -> Html {
        let on_id = ctx.link().callback(|e: InputEvent| {
            Self::Message::InsertCategoryId(Self::get_value_from_input_event(e).parse::<i64>().unwrap())
        });
        let on_name = ctx.link().callback(|e: InputEvent| {
            Self::Message::InsertCategoryName(Self::get_value_from_input_event(e))
        });
        let on_description = ctx.link().callback(|e: InputEvent| {
            Self::Message::InsertCategoryDescription(Self::get_value_from_input_event(e))
        });
        let on_image = ctx.link().callback(|e: InputEvent| {
            Self::Message::InsertCategoryImageUrl(Self::get_value_from_input_event(e))
        });

        html! {
            <>
            <div class="modal-body">
                <div class="input-group mb-3">
                    <span class="input-group-text" id="basic-addon1">{"Id de categoria"}</span>
                    <input type="text" class="form-control" placeholder="Id" aria-label="Username" aria-describedby="basic-addon1" oninput={on_id}/>
                </div>
                <div class="input-group mb-3">
                    <span class="input-group-text" id="basic-addon1">{"Nombre de la categoria"}</span>
                    <input type="text" class="form-control" placeholder="Nombre" aria-label="Username" aria-describedby="basic-addon1" oninput={on_name}/>
                </div>
                <div class="input-group mb-3">
                    <span class="input-group-text" id="basic-addon1">{"Descripción de la categoria"}</span>
                    <input type="text" class="form-control" placeholder="Descripción" aria-label="Username" aria-describedby="basic-addon1" oninput={on_description}/>
                </div>
                <div class="input-group mb-3">
                    <span class="input-group-text" id="basic-addon1">{"Url de la imagen de la categoria"}</span>
                    <input type="text" class="form-control" placeholder="Url" aria-label="Username" aria-describedby="basic-addon1" oninput={on_image}/>
                </div>
            </div>
            <div class="modal-footer">
                <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Cancelar"}</button>
                <button type="button" class="btn btn-primary" data-bs-dismiss="modal" onclick={ctx.link().callback(|_| Self::Message::AddCategory)}>{"Crear nueva categoria"}</button>
            </div> 
            </>
        }
    }
}
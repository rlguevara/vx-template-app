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

pub struct UpdtCategory {
    graphql_task: Option<GraphQLTask>,
    insert_task: Option<RequestTask>,
    category_id: i64,
    category_name: String,
    category_description: String,
    category_image_url: String,
}

pub enum UpdtCategoryMessage {
    UpdateCategoryName(String),
    UpdateCategoryDescription(String),
    UpdateCategoryImageUrl(String),
    UpdateCategory,
    CategoryUpdated(Option<update_category::ResponseData>),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub category_id: i64,
    pub category_name: String,
    pub category_description: String,
    pub category_image: String,
}


impl UpdtCategory {
    pub fn get_value_from_input_event(e: InputEvent) -> String {
        let event: Event = e.dyn_into().unwrap_throw();
        let event_target = event.target().unwrap_throw();
        let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
        target.value()
    }
}

impl Component for UpdtCategory {
    type Message = UpdtCategoryMessage;
    type Properties = Props;

    fn create (ctx: &Context<Self>) -> Self {
        Self {
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            insert_task: None,
            category_id: ctx.props().category_id,
            category_name: ctx.props().category_name.clone(),
            category_description: ctx.props().category_description.clone(),
            category_image_url: ctx.props().category_image.clone(),
        }
    }

    fn update (&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::UpdateCategoryName(category_name) => {
                self.category_name = category_name;
            }
            Self::Message::UpdateCategoryDescription(category_description) => {
                self.category_description = category_description;
            }
            Self::Message::UpdateCategoryImageUrl(category_image_url) => {
                self.category_image_url = category_image_url;
            }
            Self::Message::UpdateCategory => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = update_category::Variables {
                        id: Some(self.category_id.clone()),
                        category_name: Some(self.category_name.clone()),
                        category_description: Some(self.category_description.clone()),
                        category_image_url: Some(self.category_image_url.clone()),
                    };
                    let task = UpdateCategory::request(graphql_task, &ctx, vars, |data| {
                        Self::Message::CategoryUpdated(data)
                    });
                    self.insert_task = Some(task);
                }
            }
            Self::Message::CategoryUpdated(data) => {
                info!("CategoryUpdated: {:?}", data);
            }
        }
        true
    }

    fn view (&self, ctx: &Context<Self>) -> Html {
        let on_name = ctx.link().callback(|e: InputEvent| {
            Self::Message::UpdateCategoryName(Self::get_value_from_input_event(e))
        });
        let on_description = ctx.link().callback(|e: InputEvent| {
            Self::Message::UpdateCategoryDescription(Self::get_value_from_input_event(e))
        });
        let on_image = ctx.link().callback(|e: InputEvent| {
            Self::Message::UpdateCategoryImageUrl(Self::get_value_from_input_event(e))
        });
        let name_value = ctx.props().category_name.clone();

        html! {
            <>
            <a href="#" class="btn btn-primary m-1" data-bs-toggle="modal" data-bs-target="#exampleModal2"><i class="fa-regular fa-pen-to-square"></i> {" Actualizar categoria"}</a>

            <div class="modal fade" id="exampleModal2" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true">
                <div class="modal-dialog">
                    <div class="modal-content">
                        <div class="modal-header">
                            <h5 class="modal-title" id="exampleModalLabel">{"Editar curso"}</h5>
                            <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                        </div>
                        <div class="modal-body">
                            <div class="input-group mb-3">
                                <span class="input-group-text" id="basic-addon1">{"Nombre de la categoria"}</span>
                                <input type="text" class="form-control" aria-label="Nombre" aria-describedby="basic-addon1" oninput={on_name} placeholder={name_value}/>
                            </div>
                            <div class="input-group mb-3">
                                <span class="input-group-text" id="basic-addon1">{"Descripción de la categoria"}</span>
                                <input type="text" class="form-control" aria-label="Descripcion" aria-describedby="basic-addon1" oninput={on_description} placeholder={ctx.props().category_description.clone()}/>
                            </div>
                            <div class="input-group mb-3">
                                <span class="input-group-text" id="basic-addon1">{"Url de la imagen de la categoria"}</span>
                                <input type="text" class="form-control" aria-label="Url imagen" aria-describedby="basic-addon1" oninput={on_image} placeholder={ctx.props().category_image.clone()}/>
                            </div>
                        </div>
                        <div class="modal-footer">
                            <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Cancelar"}</button>
                            <button type="button" class="btn btn-primary" data-bs-dismiss="modal" onclick={ctx.link().callback(|_| Self::Message::UpdateCategory)}>{"Guardar cambios"}</button>
                        </div>
                    </div>
                </div>
            </div>  
            </>
        }
    }
}
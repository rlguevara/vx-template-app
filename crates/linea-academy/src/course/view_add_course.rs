use super::model_course::*;
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

#[derive(Properties, PartialEq)]
pub struct Props {
    pub c_id: i64
}

pub struct AddNewCourse {
    graphql_task: Option<GraphQLTask>,
    insert_task: Option<RequestTask>,
    course_id: i64,
    course_name: String,
    course_description: String,
    course_image_url: String,
    category_id: i64,
}

pub enum AddNewCourseMessage {
    InsertCourseId(i64),
    InsertCourseName(String),
    InsertCourseDescription(String),
    InsertCourseImageUrl(String),
    InsertCourseCategoryId(i64),
    AddCourse,
    CourseAdded(Option<add_course::ResponseData>)
}


impl AddNewCourse {
    pub fn get_value_from_input_event(e: InputEvent) -> String {
        let event: Event = e.dyn_into().unwrap_throw();
        let event_target = event.target().unwrap_throw();
        let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
        target.value()
    }
}


impl Component for AddNewCourse {
    type Message = AddNewCourseMessage;
    type Properties = Props;

    fn create (ctx: &Context<Self>) -> Self {
        Self {
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            insert_task: None,
            course_id: 0,
            course_name: "".to_string(),
            course_description: "".to_string(),
            course_image_url: "".to_string(),
            category_id: ctx.props().c_id,
        }
    }

    fn update (&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::InsertCourseId(course_id) => {
                self.course_id = course_id;
            }
            Self::Message::InsertCourseName(course_name) => {
                self.course_name = course_name;
            }
            Self::Message::InsertCourseDescription(course_description) => {
                self.course_description = course_description;
            }
            Self::Message::InsertCourseImageUrl(course_image_url) => {
                self.course_image_url = course_image_url;
            }
            Self::Message::InsertCourseCategoryId(category_id) => {
                self.category_id = category_id;
            }
            Self::Message::AddCourse => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = add_course::Variables {
                        id: Some(self.course_id.clone()),
                        course_name: Some(self.course_name.clone()),
                        course_description: Some(self.course_description.clone()),
                        course_image_url: Some(self.course_image_url.clone()),
                        category_id: Some(self.category_id.clone())
                    };
                    let task = AddCourse::request(graphql_task, &ctx, vars, |data| {
                        Self::Message::CourseAdded(data)
                    });
                    self.insert_task = Some(task);
                }
            }
            Self::Message::CourseAdded(data) => {
                info!("CourseAdded: {:?}", data);
            }
        }
        true
    }

    fn view (&self, ctx: &Context<Self>) -> Html {
        let on_id = ctx.link().callback(|e: InputEvent| {
            Self::Message::InsertCourseId(Self::get_value_from_input_event(e).parse::<i64>().unwrap())
        });
        let on_name = ctx.link().callback(|e: InputEvent| {
            Self::Message::InsertCourseName(Self::get_value_from_input_event(e))
        });
        let on_description = ctx.link().callback(|e: InputEvent| {
            Self::Message::InsertCourseDescription(Self::get_value_from_input_event(e))
        });
        let on_image = ctx.link().callback(|e: InputEvent| {
            Self::Message::InsertCourseImageUrl(Self::get_value_from_input_event(e))
        });
        let on_category_id = ctx.link().callback(|e: InputEvent| {
            Self::Message::InsertCourseCategoryId(Self::get_value_from_input_event(e).parse::<i64>().unwrap())
        });
        let obtain_category_id = ctx.props().c_id.clone().to_string();

        html! {
            // <div class="box">
            //     <div class="field">
            //         <label class="label">{"Id"}</label>
            //         <div class="control">
            //             <input class="input" type="text" placeholder="Id" oninput={on_id}/>
            //         </div>
            //     </div>
            //     <div class="field">
            //         <label class="label">{"Name"}</label>
            //         <div class="control">
            //             <input class="input" type="text" placeholder="Name" oninput={on_name}/>
            //         </div>
            //     </div>
            //     <div class="field">
            //         <label class="label">{"Description"}</label>
            //         <div class="control">
            //             <input class="input" type="text" placeholder="Description" oninput={on_description}/>
            //         </div>
            //     </div>
            //     <div class="field">
            //         <label class="label">{"Image"}</label>
            //         <div class="control">
            //             <input class="input" type="text" placeholder="Image" oninput={on_image}/>
            //         </div>
            //     </div>

            //     <div class="field">
            //         <label class="label">{"Category id"}</label>
            //         <div class="control">
            //             <input class="input" type="text" placeholder="Category id" oninput={on_category_id} c_id={obtain_category_id} />
            //         </div>
            //     </div>
            //     <div class="field is-grouped">
            //         <div class="control">
            //             <button class="button is-link" onclick={ctx.link().callback(|_| Self::Message::AddCourse)}>{"Add"}</button>
            //         </div>
            //     </div>
            // </div>
            <> 
            <div class="modal-body">
                <div class="input-group mb-3">
                    <span class="input-group-text" id="basic-addon1">{"Id del curso"}</span>
                    <input type="text" class="form-control" placeholder="Id" aria-label="Username" aria-describedby="basic-addon1" oninput={on_id}/>
                </div>
                <div class="input-group mb-3">
                    <span class="input-group-text" id="basic-addon1">{"Nombre del curso"}</span>
                    <input type="text" class="form-control" placeholder="Nombre" aria-label="Username" aria-describedby="basic-addon1" oninput={on_name}/>
                </div>
                <div class="input-group mb-3">
                    <span class="input-group-text" id="basic-addon1">{"Descripción del curso"}</span>
                    <input type="text" class="form-control" placeholder="Descripción" aria-label="Username" aria-describedby="basic-addon1" oninput={on_description}/>
                </div>
                <div class="input-group mb-3">
                    <span class="input-group-text" id="basic-addon1">{"Url de la imagen del curso"}</span>
                    <input type="text" class="form-control" placeholder="Url" aria-label="Username" aria-describedby="basic-addon1" oninput={on_image}/>
                </div>
                <div class="input-group mb-3">
                    <span class="input-group-text" id="basic-addon1">{"Id de la categoría"}</span>
                    <input type="text" class="form-control" placeholder="Id" aria-label="Username" aria-describedby="basic-addon1" c_id={ctx.props().c_id.clone().to_string()} value={obtain_category_id}/>
                </div>
            </div>
            <div class="modal-footer">
                <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Cancelar"}</button>
                <button type="button" class="btn btn-primary" data-bs-dismiss="modal" onclick={ctx.link().callback(|_| Self::Message::AddCourse)}>{"Crear curso"}</button>
            </div>    
            </>
        }
    }
}
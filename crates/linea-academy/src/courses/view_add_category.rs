use super::model_course::*;
use crate::graphql::{
    GraphQLService, GraphQLTask, Request, RequestTask};
use code_location::code_location;
use log::*;
use yew::prelude::*;
use web_sys::Event;
use web_sys::HtmlInputElement;
use web_sys::InputEvent;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;

pub struct AddNewCategory {
    graphql_task: Option<GraphQLTask>,
    insert_task: Option<RequestTask>,
    name: String,
    description: String,
    add_info: Option<add_category::ResponseData>,
}

#[derive(Debug)]
pub enum AddNewCategoryMessage {
    InsertName(String),
    InsertDescription(String),
    AddCategory,
    CategoryAdded(Option<add_category::ResponseData>),
    // Done,
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

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            insert_task: None,
            name: "".to_string(),
            description: "".to_string(),
            add_info: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::InsertName(name) => {
                self.name = name;
            }
            Self::Message::InsertDescription(description) => {
                self.description = description;
            }
            Self::Message::AddCategory => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = add_category::Variables {
                        id: Some(36),
                        name: self.name.clone(),
                        description: self.description.clone(),
                    };
                    let task = AddCategory::request(graphql_task, &ctx, vars, |data| {
                        Self::Message::CategoryAdded(data)
                    });
                    self.insert_task = Some(task);
                }
            }
            Self::Message::CategoryAdded(data) => {
                info!("CategoryAdded: {:?}", data);
                self.add_info = data;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_name = ctx.link().callback(|e: InputEvent| {
            Self::Message::InsertName(Self::get_value_from_input_event(e))
        });
        let on_description = ctx.link().callback(|e: InputEvent| {
            Self::Message::InsertDescription(Self::get_value_from_input_event(e))
        });
        let info = self.add_info.clone();
        let display_name: String = self.name.clone();
        let display_description: String = self.description.clone();
        html! {
            <div class="box">
                <div class="field">
                    <label class="label">{"Name"}</label>
                    <div class="control">
                        <input class="input" type="text" placeholder="Name" oninput={on_name}/>
                    </div>
                </div>
                <div class="field">
                    <label class="label">{"Description"}</label>
                    <div class="control">
                        <input class="input" type="text" placeholder="Description" oninput={on_description}/>
                    </div>
                </div>
                <div class="field is-grouped">
                    <div class="control">
                        <button class="button is-link" onclick={ctx.link().callback(|_| Self::Message::AddCategory)}>{"Add"}</button>
                    </div>
                </div>
                <p>{format!("{:?}", info)}</p>
                <p>{format!("{:?}", display_name)}</p>
                <p>{format!("{:?}", display_description)}</p>
            </div>
        }
    }
}
















































// impl Component for CourseView {
//     type Message = CourseViewMessage;
//     type Properties = ();

//     fn create(_ctx: &Context<Self>) -> Self {
//         Self {
//             graphql_task: Some(GraphQLService::connect(&code_location!())),
//             req_task: None,
//             // testi: None,
//             // test2: None,
//             // testi2: None,
//             // sub_task: None,
//         }
//     }

//     fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
//         // info!("{:?}", msg);
//         match msg {
//             Self::Message::AddCategory => {
//                 if let Some(graphql_task) = self.graphql_task.as_mut() {
//                     let vars = add_category::Variables {
//                         name: "testi".to_string(),
//                         description: "testi".to_string(),
//                     };
//                     let task = AddCategory::request(graphql_task, &ctx, vars, |data| {
//                         // Self::Message::CategoryAdded(data)
//                     });
//                     self.req_task = Some(task);
//                 }
//             }
//             Self::Message::CategoryAdded(data) => {
//                 info!("CategoryAdded: {:?}", data);
//             }
//             // Self::Message::TimeAdd => {
//             //     if let Some(graphql_task) = self.graphql_task.as_mut() {
//             //         let vars = time_add::Variables {
//             //             time: wasm_utc_now().to_rfc3339(),
//             //         };
//             //         let task = TimeAdd::request(graphql_task, &ctx, vars, |data| {
//             //             Self::Message::TimeAdded(data)
//             //         });
//             //         self.req_task = Some(task);
//             //     }
//             // }
//             // Self::Message::TimeAdded(data) => {
//             //     info!("TimeAdded123: {:?}", data);
//             //     self.testi = data;
//             // }

//             // Self::Message::TimeDataReq => {
//             //     if let Some(graphql_task) = self.graphql_task.as_mut() {
//             //         let vars = time_data::Variables { };
//             //         let task = TimeData::request(graphql_task, &ctx, vars, |data| {
//             //             Self::Message::TimeDataResponse(data)
//             //         });
//             //         self.test2 = Some(task);
//             //     }
//             // }
//             // Self::Message::TimeDataResponse(data) => {
//             //     info!("TimeDataResponse: {:?}", data);
//             //     self.testi2 = data;
//             // }
//             // Self::Message::OnTimeAdded(data) => {
//             //     info!("OnTimeAdded: {:?}", data)
//             // }
//             // Self::Message::SubscriptionEnable => {
//             //     self.graphql_task = Some(GraphQLService::connect(&code_location!()))
//             // }
//             // Self::Message::SubscriptionDisable => self.graphql_task = None,
//             // Self::Message::Subscribe => {
//             //     if let Some(graphql_task) = self.graphql_task.as_mut() {
//             //         let vars = on_time_added::Variables {};
//             //         let sub_task = OnTimeAdded::subscribe(graphql_task, &ctx, vars, |data| {
//             //             Self::Message::OnTimeAdded(data)
//             //         });
//             //         self.sub_task = Some(sub_task);
//             //     }
//             // }
//             // Self::Message::Unsubscribe => {
//             //     self.sub_task = None;
//             // }
//         }
//         true
//     }

//     fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
//         // info!("{:?}", props);
//         true
//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {
//         // let graphql_task = if self.graphql_task.is_some() {
//         //     html! { <button class="button is-dark my-1" onclick={ctx.link().callback(|_| Self::Message::SubscriptionDisable)}>{"Subscription Disable"}</button> }
//         // } else {
//         //     html! { <button class="button is-dark my-1" onclick={ctx.link().callback(|_| Self::Message::SubscriptionEnable)}>{"Subscription Enable"}</button> }
//         // };

//         // let subscribed = if self.sub_task.is_some() {
//         //     html! { <button class="button is-dark my-1" onclick={ctx.link().callback(|_| Self::Message::Unsubscribe)}>{"Unsubscribe"}</button> }
//         // } else {
//         //     html! { <button class="button is-dark my-1" onclick={ctx.link().callback(|_| Self::Message::Subscribe)}>{"Subscribe"}</button> }
//         // };
//         // let position_add = ctx.link().callback(|_| Self::Message::TimeAdd);
//         // let test_time_added = ctx.link().callback(|_| Self::Message::TimeAdded(None));
//         // let testi2 = self.testi.clone();
//         // let test_time_data = ctx.link().callback(|_| Self::Message::TimeDataReq);
//         // let testi3 = self.testi2.clone();
//         // let testi3 = self.test2.clone();
//         html! {
//             <>
//                 // <button class="button is-dark my-1" onclick={position_add}>{"Send Location"}</button>
//                 // <button class="button is-dark my-1" onclick={test_time_added}>{"Test Time Added"}</button>
//                 // <p>{format!("{:?}", testi2)}</p>
//                 // <button class="button is-dark my-1" onclick={test_time_data}>{"Test Time Data"}</button>
//                 // <p>{format!("{:?}", testi3)}</p>
//                 // <br />
//                 // { graphql_task }
//                 <br />
//                 // { subscribed }
//             </>
//         }
//     }
// }

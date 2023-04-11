use super::model_category::*;
use yew::prelude::*;
use yew_router::{prelude::*, navigator};
use log::*;

use crate::graphql::{
    GraphQLService, GraphQLTask, Request, RequestTask};
use code_location::code_location;
use crate::components::router::Route;
use crate::components::categoryitem::CategoryItem;

pub struct ShowCategoryList {
    graphql_task: Option<GraphQLTask>,
    req_task: Option<RequestTask>,
    category: Option<show_category::ResponseData>
}

pub enum ShowCategoryMessage {
    ShowCategoryData,
    CategoryReceived(Option<show_category::ResponseData>)
}

impl Component for ShowCategoryList {
    type Message = ShowCategoryMessage;
    type Properties = ();

    fn create (ctx: &Context<Self>) -> Self {
        let show_data = ctx.link().send_message(Self::Message::ShowCategoryData);
        Self {
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            req_task: None,
            category: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::ShowCategoryData => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = show_category::Variables {
                    };
                    let task = ShowCategory::request(graphql_task, &ctx, vars, |data| {
                        Self::Message::CategoryReceived(data)
                    });
                    self.req_task = Some(task);
                }
            }
            Self::Message::CategoryReceived(data) => {
                self.category = data.clone();
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        // let category = self.category.clone();
        // let navigator = Some(ctx.link().navigator()).unwrap();
        let list_category = self.category.clone().and_then(|data| Some(data.lr_academy_category)).unwrap_or_default().iter().map(|category| {
            // let category_id_data = category.id.clone();
            // let navigator = navigator.clone().unwrap();
            // let navigator = ctx.link().navigator().unwrap();
            // let onclick = Callback::from(move |_| navigator.push(&Route::Courses { id: category_id_data}));
            html!{
                // <div>
                    <CategoryItem title={category.category_name.clone()} description={category.category_description.clone()} category_id={category.id.clone()} />
                    // <div>{&category.category_name}</div>
                    // <div>{&category.category_description}</div>
                    // <div>{&category.category_image_url}</div>
                    // <div>{category.id}</div>
                    // <div>{"Hello?"}</div>
                    // <button class="button is-link" onclick={onclick}>{"Read more..."}</button>
                // </div>
            }
            }).collect::<Html>();
        html! {
            {list_category}
        }
    }

}

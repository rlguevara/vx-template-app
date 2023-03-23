use super::model_category::*;
use yew::prelude::*;

use crate::graphql::{
    GraphQLService, GraphQLTask, Request, RequestTask};
use code_location::code_location;
use crate::components::categoryinfo::CategoryInfo;

pub struct CategoryById {
    graphql_task: Option<GraphQLTask>,
    req_task: Option<RequestTask>,
    category: Option<show_category_by_id::ResponseData>,
    category_id: i64,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub category_id: i64,
}

pub enum CategoryByIdMessage {
    ShowCategoryDataByID,
    CategoryReceived(Option<show_category_by_id::ResponseData>)
}

impl Component for CategoryById {
    type Message = CategoryByIdMessage;
    type Properties = Props;

    fn create (ctx: &Context<Self>) -> Self {
        let show_data = ctx.link().send_message(Self::Message::ShowCategoryDataByID);
        Self {
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            req_task: None,
            category: None,
            category_id: ctx.props().category_id,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::ShowCategoryDataByID => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = show_category_by_id::Variables {
                        eq: Some(ctx.props().category_id),
                    };
                    let task = ShowCategoryById::request(graphql_task, &ctx, vars, |data| {
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
        // let show_category_data = ctx.link().callback(|_| Self::Message::ShowCategoryDataByID);
        let list_category = self.category.clone().and_then(|data| Some(data.lr_academy_category)).unwrap_or_default().iter().map(|category| {
            html!{
                <div>
                    <CategoryInfo category_name={category.category_name.clone()}
                                    category_description={category.category_description.clone()}
                                    category_image={category.category_image_url.clone()}
                                    category_id={category.id}
                    />
                </div>
            }
            }).collect::<Html>();
        html! {
            // <div>
            // <button class="button is-dark my-1" onclick={show_category_data}>{"Show category"}</button>
            {list_category}
            // </div>
        }
    }

}
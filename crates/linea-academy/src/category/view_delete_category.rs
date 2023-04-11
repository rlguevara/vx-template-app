use super::model_category::*;
use yew::prelude::*;

use crate::graphql::{
    GraphQLService, GraphQLTask, Request, RequestTask};
use code_location::code_location;
use log::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub category_id: i64
}

pub struct DelCategory {
    graphql_task: Option<GraphQLTask>,
    insert_task: Option<RequestTask>,
    category_id: i64,
}

pub enum DelCategoryMessage {
    DeleteCategory,
    CategoryDeleted(Option<delete_category::ResponseData>)
}

impl Component for DelCategory {
    type Message = DelCategoryMessage;
    type Properties = Props;

    fn create (ctx: &Context<Self>) -> Self {
        Self {
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            insert_task: None,
            category_id: ctx.props().category_id,
        }
    }

    fn update (&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::DeleteCategory => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = delete_category::Variables {
                        id: Some(self.category_id.clone()),
                    };
                    let task = DeleteCategory::request(graphql_task, &ctx, vars, |data| {
                        Self::Message::CategoryDeleted(data)
                    });
                    self.insert_task = Some(task);
                }
            }
            Self::Message::CategoryDeleted(data) => {
                info!("CategoryDeleted: {:?}", data);
            }
        }
        true
    }

    fn view (&self, ctx: &Context<Self>) -> Html {
        let show_id = ctx.props().category_id;
        html! {
            <>
            <a href="#" class="btn btn-danger m-1 float-right" onclick={ctx.link().callback(|_| Self::Message::DeleteCategory)}><i class="fa-solid fa-trash"></i> {" Borrar categoria"}</a>
            </>
        }
    }
}
use async_graphql::Object;

use super::{model::NHentai, service::get_nhentai_by_id};

#[derive(Default)]
pub struct NHentaiQuery;

#[Object]
impl NHentaiQuery {
    pub async fn get_nhentai_by_id(&self, id: i32) -> NHentai {
        get_nhentai_by_id(id).await
    }
}
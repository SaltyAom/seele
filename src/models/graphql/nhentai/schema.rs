use async_graphql::Object;

use super::{model::{NHentai, NHentaiGroup}, service::*};

#[derive(Default)]
pub struct NHentaiQueryRoot;

#[Object]
impl NHentaiQueryRoot {
    pub async fn nhentai(&self) -> NHentaiQuery {
        NHentaiQuery::default()
    }
}

#[derive(Default)]
pub struct NHentaiQuery;

#[Object]
impl NHentaiQuery {
    pub async fn by(&self, id: u32) -> NHentai {
        get_nhentai_by_id(id).await
    }

    pub async fn search(&self, find: String, #[graphql(default = 1)] page: u16) -> NHentaiGroup {
        search_nhentai(find, page).await
    }
}

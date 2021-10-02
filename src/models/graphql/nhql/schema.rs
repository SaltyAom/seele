use async_graphql::Object;

use super::{model::{NHResponse, NHSearchResponse}, service::*};

#[derive(Default)]
pub struct NhqlQueryRoot;

#[Object]
impl NhqlQueryRoot {
    pub async fn nhql(&self) -> NhqlQuery {
        NhqlQuery::default()
    }
}

#[derive(Default)]
pub struct NhqlQuery;

#[Object]
impl NhqlQuery {
    pub async fn by(&self, id: u32) -> NHResponse {
        get_nhql(id).await
    }

    pub async fn search(&self, with: String, #[graphql(default = 1)] page: u16) -> NHSearchResponse {
        search_nhql(with, page).await
    }
}

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

fn empty_vec() -> Vec<String> {
    vec![]
}

#[Object]
impl NHentaiQuery {
    pub async fn by(&self, id: u32) -> NHentai {
        get_nhentai_by_id(id).await
    }

    pub async fn search(
        &self, 
        #[graphql(default = "")] with: String,
        #[graphql(default = 1)] page: u16, 
        #[graphql(default_with = "empty_vec()")] includes: Vec<String>,
        #[graphql(default_with = "empty_vec()")] excludes: Vec<String>,
        #[graphql(default_with = "empty_vec()")] tags: Vec<String>,
        #[graphql(default_with = "empty_vec()")] artists: Vec<String>,
    ) -> NHentaiGroup {
        search_nhentai(with, page, includes, excludes, tags, artists).await
    }
}

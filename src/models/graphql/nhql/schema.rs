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

fn empty_vec() -> Vec<String> {
    vec![]
}

#[derive(Default)]
pub struct NhqlQuery;

#[Object]
impl NhqlQuery {
    pub async fn by(&self, id: u32) -> NHResponse {
        get_nhql(id).await
    }

    pub async fn search(
        &self, 
        #[graphql(default = "")] with: String,
        #[graphql(default = 1)] page: u16,
        #[graphql(default_with = "empty_vec()")] includes: Vec<String>,
        #[graphql(default_with = "empty_vec()")] excludes: Vec<String>,
        #[graphql(default_with = "empty_vec()")] tags: Vec<String>,
        #[graphql(default_with = "empty_vec()")] artists: Vec<String>
    ) -> NHSearchResponse {
        search_nhql(with, page, includes, excludes, tags, artists).await
    }
}

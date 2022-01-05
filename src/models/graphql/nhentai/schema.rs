use async_graphql::Object;

use super::{
    model::{
        // MultipleNHentaiResponse, 
        NHentai, 
        NHentaiGroup
    },
    service::*,
};

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

    // ? Disabled due to nHentai NGINX rate limit
    // pub async fn multiple(&self, id: Vec<u32>) -> MultipleNHentaiResponse {
    //     let mut dedup_id = id.clone();
    //     dedup_id.sort();
    //     dedup_id.dedup();

    //     if dedup_id.len() != id.len() {
    //         return MultipleNHentaiResponse {
    //             success: false,
    //             error: Some("Ids have to be unique"),
    //             data: vec![],
    //         };
    //     }

    //     if id.len() > 25 {
    //         return MultipleNHentaiResponse {
    //             success: false,
    //             error: Some("Ids is limit to 25 per request"),
    //             data: vec![],
    //         };
    //     }

    //     let hentais = get_nhentais_by_id(id).await;

    //     MultipleNHentaiResponse {
    //         success: true,
    //         error: None,
    //         data: hentais,
    //     }
    // }

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

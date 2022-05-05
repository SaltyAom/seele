use async_graphql::Object;

use crate::models::graphql::nhql::model::NhqlChannel;

use super::{
    model::{MultipleNHentaiResponse, NHentai, NHentaiGroup},
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

#[Object]
/// nHentai Query
/// 
/// Same format as nHentai API
impl NHentaiQuery {
    /// Get nHentai by ID (6 digits code)
    pub async fn by(
        &self,
        id: u32,
        #[graphql(default_with = "NhqlChannel::HifuminFirst")] channel: NhqlChannel,
    ) -> NHentai {
        get_nhentai_by_id(id, channel).await
    }

    /// Get multiple nHentai by ID (6 digits code)
    /// 
    /// - IDs must be unique
    /// - Maximum 25 IDs per batch
    /// - Only available for HifuminFirst channel
    pub async fn multiple(&self, id: Vec<u32>) -> MultipleNHentaiResponse {
        let mut dedup_id = id.clone();
        dedup_id.sort();
        dedup_id.dedup();

        if dedup_id.len() != id.len() {
            return MultipleNHentaiResponse {
                success: false,
                error: Some("Ids have to be unique"),
                data: vec![],
            };
        }

        if id.len() > 25 {
            return MultipleNHentaiResponse {
                success: false,
                error: Some("Ids is limit to 25 per request"),
                data: vec![],
            };
        }

        let hentais = get_nhentais_by_id(id).await;

        MultipleNHentaiResponse {
            success: true,
            error: None,
            data: hentais,
        }
    }

    /// Search from nHentai
    pub async fn search(
        &self,
        #[graphql(default = "")] with: String,
        #[graphql(default = 1)] page: u16,
        #[graphql(default_with = "vec![]")] includes: Vec<String>,
        #[graphql(default_with = "vec![]")] excludes: Vec<String>,
        #[graphql(default_with = "vec![]")] tags: Vec<String>,
        #[graphql(default_with = "vec![]")] artists: Vec<String>,
        #[graphql(default_with = "NhqlChannel::HifuminFirst")] channel: NhqlChannel
    ) -> NHentaiGroup {
        search_nhentai(channel, with, page, includes, excludes, tags, artists).await
    }
}

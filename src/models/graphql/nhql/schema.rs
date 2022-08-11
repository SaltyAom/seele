use async_graphql::Object;

use crate::services::SearchOption;

use super::{
    service::*,
    model::{
        NHResponse, 
        NHSearchResponse, 
        NhqlChannel,
        MultipleNHResponse
    },
};

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
/// Nhql (nHentai API)
/// 
/// Easier formatted data, ready to used out of the box
impl NhqlQuery {
    /// Get nHentai by ID (6 digits code)
    pub async fn by(
        &self, 
        id: u32,
        #[graphql(default_with = "NhqlChannel::HifuminFirst")] channel: NhqlChannel
    ) -> NHResponse {
        get_nhql(id, channel).await
    }

    /// Get multiple nHentai by ID (6 digits code)
    /// 
    /// - IDs must be unique
    /// - Maximum 25 IDs per batch
    /// - Only available for HifuminFirst channel
    pub async fn multiple(&self, id: Vec<u32>) -> MultipleNHResponse {
        let mut dedup_id = id.clone();
        dedup_id.sort();
        dedup_id.dedup();

        if dedup_id.len() != id.len() {
            return MultipleNHResponse {
                success: false,
                error: Some("Ids have to be unique"),
                data: vec![],
            };
        }

        if id.len() > 25 {
            return MultipleNHResponse {
                success: false,
                error: Some("Ids is limit to 25 per request"),
                data: vec![]
            }
        }

        MultipleNHResponse {
            success: true,
            error: None,
            data: get_multiple_nhql(id).await
        }
    }

    /// Search from nHentai
    pub async fn search(
        &self, 
        #[graphql(default = "")] with: String,
        #[graphql(default = 1)] page: u16,
        #[graphql(default_with = "vec![]")] includes: Vec<String>,
        #[graphql(default_with = "vec![]")] excludes: Vec<String>,
        #[graphql(default_with = "NhqlChannel::HifuminFirst")] channel: NhqlChannel
    ) -> NHSearchResponse {
        search_nhql(SearchOption { 
            keyword: with,
            channel, 
            batch: page,
            includes,
            excludes
        }).await
    }
}

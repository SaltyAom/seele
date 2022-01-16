use async_graphql::Object;

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

fn empty_vec() -> Vec<String> {
    vec![]
}

fn default_channel() -> NhqlChannel {
    NhqlChannel::Hifumin
}

#[derive(Default)]
pub struct NhqlQuery;

#[Object]
impl NhqlQuery {
    pub async fn by(
        &self, 
        id: u32,
        #[graphql(default_with = "default_channel()")] channel: NhqlChannel
    ) -> NHResponse {
        get_nhql(id, channel).await
    }

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

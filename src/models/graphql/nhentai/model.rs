use serde::{ Serialize, Deserialize };
use serde_aux::prelude::*;

use async_graphql::*;

use crate::models::graphql::nhql::model::{NhqlCommentOrder, NhqlChannel};

use super::service::{ get_comment_range, get_related };

#[derive(Default)]
pub struct NHentaiQuery;

#[derive(Serialize, Deserialize, Clone, SimpleObject)]
#[graphql(complex)]
pub struct NHentai {
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub id: Option<u32>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub media_id: Option<u32>,
    pub title: NHentaiTitle,
    pub images: NHentaiImages,
    pub scanlator: Option<String>,
    pub upload_date: Option<u32>,
    pub tags: NHentaiTags,
    pub num_pages: Option<u16>,
    pub num_favorites: Option<u32>,
    pub channel: NhqlChannel
}

#[derive(Serialize, Deserialize, Clone, SimpleObject)]
pub struct InternalNHentai {
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub id: Option<u32>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub media_id: Option<u32>,
    pub title: NHentaiTitle,
    pub images: NHentaiImages,
    pub scanlator: Option<String>,
    pub upload_date: Option<u32>,
    pub tags: NHentaiTags,
    pub num_pages: Option<u16>,
    pub num_favorites: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone, SimpleObject)]
pub struct MultipleNHentaiResponse {
    pub success: bool,
    pub error: Option<&'static str>,
    pub data: Vec<NHentai>
}

#[derive(Serialize, Deserialize, Clone, SimpleObject)]
pub struct NHentaiTitle {
    pub english: Option<String>,
    pub japanese: Option<String>,
    pub pretty: Option<String>
}

#[ComplexObject]
impl NHentai {
    pub async fn comments(
        &self, 
        from: Option<u32>,     
        to: Option<u32>,
        batch: Option<u32>,
        batch_by: Option<u32>,
        order_by: Option<NhqlCommentOrder>
    ) -> Vec<NHentaiComment> {
        get_comment_range(self.id.unwrap(), from, to, batch, batch_by ,order_by, self.channel).await
    }

    pub async fn related(
        &self, 
    ) -> Vec<NHentai> {
        get_related(self.id.unwrap()).await
    }
}

#[derive(Serialize, Deserialize, Clone, SimpleObject)]
pub struct NHentaiImages {
    pub pages: NHentaiPages,
    pub cover: NHentaiPage,
    pub thumbnail: NHentaiPage
}

#[derive(Serialize, Deserialize, Clone, SimpleObject)]
pub struct NHentaiPage {
    pub t: Option<String>,
    pub w: Option<u16>,
    pub h: Option<u16>
}

pub type NHentaiPages = Vec<NHentaiPage>;

#[derive(Serialize, Deserialize, Clone, SimpleObject)]
pub struct NHentaiTag {
    pub id: u32,
    pub r#type: String,
    pub name: String,
    pub url: String,
    pub count: u32
}

pub type NHentaiTags = Vec<NHentaiTag>;

#[derive(Serialize, Deserialize, Clone, SimpleObject)]
pub struct NHentaiGroup {
    pub result: Vec<NHentai>,
    pub num_pages: Option<u16>,
    pub per_page: Option<u8>
}

#[derive(Serialize, Deserialize, Clone, SimpleObject)]
pub struct InternalNHentaiGroup {
    pub result: Vec<InternalNHentai>,
    pub num_pages: Option<u16>,
    pub per_page: Option<u8>
}

#[derive(Serialize, Deserialize, Clone, SimpleObject)]
pub struct NHentaiComment {
    pub id: u32,
    pub gallery_id: u32,
    pub poster: NHentaiCommentPoster,
    pub post_date: u32,
    pub body: String
}

#[derive(Serialize, Deserialize, Clone, SimpleObject)]
pub struct NHentaiCommentPoster {
    pub id: u32,
    pub username: String,
    pub slug: String,
    pub avatar_url: String,
    pub is_superuser: bool,
    pub is_staff: bool
}

#[derive(Serialize, Deserialize, Clone, SimpleObject)]
pub struct NHentaiRelated {
    pub result: Vec<NHentai>
}
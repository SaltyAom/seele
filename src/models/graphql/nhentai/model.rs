use serde::{ Serialize, Deserialize };

use async_graphql::*;

#[derive(Default)]
pub struct NHentaiQuery;


#[derive(Serialize, Deserialize, Clone, SimpleObject)]
pub struct NHentai {
    pub id: Option<u32>,
    pub media_id: Option<String>,
    pub title: NHentaiTitle,
    pub images: NHentaiImages,
    pub scanlator: Option<String>,
    pub upload_date: Option<u32>,
    pub tags: NHentaiTags,
    pub num_pages: Option<u16>,
    pub num_favorites: Option<u32>
}

#[derive(Serialize, Deserialize, Clone, SimpleObject)]
pub struct NHentaiTitle {
    pub english: Option<String>,
    pub japanese: Option<String>,
    pub pretty: Option<String>
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
    pub num_pages: u16,
    pub per_page: u8
}

use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct NHResponse {
    pub success: bool,
    pub info: String,
    pub data: NHApi
}

#[derive(Serialize, Clone)]
pub struct NHSearchResponse {
    pub success: bool,
    pub info: String,
    pub data: NHApiSearch
}

#[derive(Serialize, Clone)]
pub struct NHApi {
    pub id: i32,
    pub title: NHApiTitle,
    pub images: NHApiImages,
    pub info: NHApiInfo,
    pub metadata: NHApiMetadata
}

#[derive(Serialize, Clone)]
pub struct NHApiTitle {
    pub display: String,
    pub english: String,
    pub japanese: String
}

#[derive(Serialize, Clone)]
pub struct NHApiImages {
    pub pages: NHApiPages,
    pub cover: NHApiPage
}

#[derive(Serialize, Clone)]
pub struct NHApiPage {
    pub link: String,
    pub info: NHApiPageInfo
}

#[derive(Serialize, Clone)]
pub struct NHApiPageInfo {
    pub r#type: String,
    pub width: i32,
    pub height: i32
}

pub type NHApiPages = Vec<NHApiPage>;

#[derive(Serialize, Clone)]
pub struct NHApiInfo {
    pub amount: i32,
    pub favorite: i32,
    pub upload: NHApiInfoUpload
}

#[derive(Serialize, Clone)]
pub struct NHApiInfoUpload {
    pub original: i32,
    pub parsed: String
}

#[derive(Serialize, Clone)]
pub struct NHApiMetadata {
    pub artist: NHApiArtist,
    pub tags: NHApiTags,
    pub language: String
}

#[derive(Serialize, Clone)]
pub struct NHApiArtist {
    pub name: String,
    pub count: i32,
    pub url: String
}

#[derive(Serialize, Clone)]
pub struct NHApiTag {
    pub name: String,
    pub count: i32,
    pub url: String
}

pub type NHApiTags = Vec<NHApiTag>;
pub type NHApiSearch = Vec<NHApi>;
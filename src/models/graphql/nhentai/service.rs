use super::{ model::*, constant::* };
use crate::services::request::get;

use cached::proc_macro::cached;

#[cached]
pub async fn get_nhentai_by_id(id: u32) -> NHentai {
    let response = get::<NHentai>(
        format!("https://nhentai.net/api/gallery/{}", id)
    );

    if let Ok(nhentai) = response.await {
        nhentai
    } else {
        EMPTY_NHENTAI_DATA
    }
}

#[cached]
pub async fn search_nhentai(search: String, page: u16) -> NHentaiGroup {
    let response = get::<NHentaiGroup>(
        format!("https://nhentai.net/api/galleries/search?query={}&page={}", search, page)
    );

    if let Ok(nhentai) = response.await {
        nhentai
    } else {
        EMPTY_NHENTAI_GROUP
    }
}

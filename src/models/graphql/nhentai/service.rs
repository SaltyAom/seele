use super::model::*;
use crate::services::request::get;

use cached::proc_macro::cached;

#[cached]
pub async fn get_nhentai_by_id(id: i32) -> NHentai {
    let response = get::<NHentai>(
        format!("{}/{}", "https://nhentai.net/api/gallery", id)
    );

    if let Ok(nhentai) = response.await {
        nhentai
    } else {
        NHentai {
            id: None,
            title: NHentaiTitle {
                english: None,
                japanese: None,
                pretty: None
            },
            media_id: None,
            images: NHentaiImages {
                pages: vec![],
                cover: NHentaiPage {
                    t: None, 
                    w: None, 
                    h: None
                },
                thumbnail: NHentaiPage { 
                    t: None, 
                    w: None, 
                    h: None 
                }
            },
            scanlator: None,
            upload_date: None,
            tags: vec![],
            num_pages: None,
            num_favorites: None
        }
    }
}

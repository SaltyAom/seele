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
pub async fn search_nhentai(
    search: String, 
    page: u16, 
    includes: Vec<String>, 
    excludes: Vec<String>,
    tags: Vec<String>,
    artists: Vec<String>,
) -> NHentaiGroup {
    let mut query = search + " ";

    for tag in tags.into_iter() {
        query += &format!("tag:\"{}\"", tag);
    }

    for include in includes.into_iter() {
        query += &format!("+\"{}\"", include);
    }

    for exclude in excludes.into_iter() {
        query += &("+-".to_owned() + &exclude);
    }

    if artists.len() == 1 {
        query += &format!("artist:\"{}\"", artists[0]);
    } else {
        for artist in artists.into_iter() {
            query += &format!("artist:{}", artist);
        }
    }

    let response = get::<NHentaiGroup>(
        format!("https://nhentai.net/api/galleries/search?query={}&page={}", query, page)
    );

    match response.await {
        Ok(nhentai) => nhentai,
        Err(_error) => {
            EMPTY_NHENTAI_GROUP
        }
    }
}

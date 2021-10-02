use super::{
    model::*,
    utils::*,
    super::nhentai::service::*,
};

use cached::proc_macro::cached;

#[cached]
pub async fn get_nhql(id: u32) -> NHResponse {
    let nhentai = get_nhentai_by_id(id).await;

    if nhentai.id.is_none() {
        return NHResponse {
            success: false,
            info: Some("Not found"),
            data: None
        }
    }

    NHResponse {
        success: true,
        info: None,
        data: Some(map_nhql(nhentai))
    }
}

#[cached]
pub async fn search_nhql(find: String, page: u16) -> NHSearchResponse {
    let nhentais = search_nhentai(find, page).await;

    if nhentais.result.len() == 0 {
        return NHSearchResponse {
            success: false,
            info: Some("Not found"),
            data: vec![]
        }
    }

    NHSearchResponse {
        success: true,
        info: None,
        data: nhentais.result.into_iter().map(|nhentai| map_nhql(nhentai)).collect()
    }
}

use super::{
    model::*,
    utils::*,
    super::nhentai::service::{ 
        get_nhentai_by_id,
        search_nhentai,
        get_comment_range,
        get_related
    }
};

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

pub async fn search_nhql(
    find: String, 
    page: u16, 
    includes: Vec<String>, 
    excludes: Vec<String>,
    tags: Vec<String>,
    artists: Vec<String>
) -> NHSearchResponse {
    let nhentais = search_nhentai(find, page, includes, excludes, tags, artists).await;

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

pub async fn get_nhql_comment(
    id: u32,
    from: Option<u32>,
    to: Option<u32>,
    batch: Option<u32>,
    batch_by: Option<u32>
) -> Vec<NhqlComment> {
    let comments = get_comment_range(id, from, to, batch, batch_by).await;

    comments.into_iter().map(|comment| {
        NhqlComment {
            id: comment.id,
            user: NhqlUser {
                id: comment.poster.id,
                username: comment.poster.username,
                slug: format!("https://nhentai.net/users/{}/{}", comment.poster.id, comment.poster.slug),
                avatar: format!("https://i.nhentai.net/{}", comment.poster.avatar_url)
            },
            created: comment.post_date,
            body: comment.body,
        }
    }).collect()
}

pub async fn get_nhql_related(id: u32,) -> Vec<Nhql> {
    let related = get_related(id).await;

    related.into_iter().map(|nhentai| map_nhql(nhentai)).collect()
}
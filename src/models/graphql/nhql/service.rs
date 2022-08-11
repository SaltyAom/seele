use crate::services::SearchOption;

use super::{
    super::nhentai::service::{
        get_comment_range, 
        get_nhentai_by_id, 
        get_nhentais_by_id, 
        get_related, 
        search_nhentai
    },
    model::*,
    utils::*,
};

pub async fn get_multiple_nhql(id: Vec<u32>) -> Vec<NHResponse> {
    let nhentais = get_nhentais_by_id(id).await;

    nhentais
        .into_iter()
        .map(move |nhentai| {
            if nhentai.id.is_some() {
                NHResponse {
                    success: true,
                    error: None,
                    data: Some(map_nhql(nhentai))
                }
            } else {
                NHResponse {
                    success: false,
                    error: Some("Not found"),
                    data: None
                }
            }
        })
        .collect::<Vec<NHResponse>>()
}

pub async fn get_nhql(id: u32, channel: NhqlChannel) -> NHResponse {
    let nhentai = get_nhentai_by_id(id, channel).await;

    if nhentai.id.is_none() {
        return NHResponse {
            success: false,
            error: Some("Not found"),
            data: None,
        };
    }

    NHResponse {
        success: true,
        error: None,
        data: Some(map_nhql(nhentai)),
    }
}

pub async fn search_nhql(
    search: SearchOption
) -> NHSearchResponse {
    let nhentais = search_nhentai(search).await;

    if nhentais.result.len() == 0 {
        return NHSearchResponse {
            success: false,
            error: Some("Not found"),
            data: vec![],
        };
    }

    NHSearchResponse {
        success: true,
        error: None,
        data: nhentais
            .result
            .into_iter()
            .map(|nhentai| map_nhql(nhentai))
            .collect(),
    }
}

pub async fn get_nhql_comment(
    id: u32,
    from: Option<u32>,
    to: Option<u32>,
    batch: Option<u32>,
    batch_by: Option<u32>,
    order_by: Option<NhqlCommentOrder>,
    channel: NhqlChannel
) -> Vec<NhqlComment> {
    get_comment_range(id, from, to, batch, batch_by, order_by, channel)
        .await
        .into_iter()
        .map(|comment| NhqlComment {
            id: comment.id,
            user: NhqlUser {
                id: comment.poster.id,
                username: comment.poster.username,
                slug: format!(
                    "https://nhentai.net/users/{}/{}",
                    comment.poster.id, comment.poster.slug
                ),
                avatar: format!("https://i.nhentai.net/{}", comment.poster.avatar_url),
            },
            created: comment.post_date,
            comment: comment.body,
        })
        .collect()
}

pub async fn get_nhql_related(id: u32, channel: NhqlChannel) -> Vec<Nhql> {
    let related = get_related(id, channel).await;

    related
        .into_iter()
        .map(|nhentai| map_nhql(nhentai))
        .collect()
}

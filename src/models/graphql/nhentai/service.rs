use super::{
    constant::*, 
    model::*, 
    super::nhql::model::NhqlCommentOrder
};
use crate::services::request::get;

use cached::proc_macro::cached;

// use futures::{stream, StreamExt};
// use tokio::time::{sleep, Duration};

// const PARALLEL_REQUESTS: usize = 6;

// pub async fn get_nhentais_by_id(id: Vec<u32>) -> Vec<NHentai> {
//     let limit = id.len();
//     let (tx, mut rx) = tokio::sync::mpsc::channel::<NHentai>(limit);

//     let responses = stream::iter(id)
//         .map(|id| {
//             tokio::spawn(async move {
//                 sleep(Duration::from_millis(100)).await;
//                 get_nhentai_by_id(id).await
//             })
//         })
//         .buffered(PARALLEL_REQUESTS);

//     responses
//         .for_each(|res| async {
//             match tx.send(res.unwrap_or(EMPTY_NHENTAI_DATA)).await {
//                 Ok(_a) => {},
//                 Err(_e) => {}
//             }
//         })
//         .await;

//     let mut hentais: Vec<NHentai> = vec![];
//     while let Some(nhentai) = rx.recv().await {
//         hentais.push(nhentai);

//         if hentais.len() >= limit - 1 {
//             break
//         }
//     }

//     hentais
// }

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

    let response = get::<NHentaiGroup>(format!(
        "https://nhentai.net/api/galleries/search?query={}&page={}",
        query, page
    ));

    match response.await {
        Ok(nhentai) => nhentai,
        Err(_error) => EMPTY_NHENTAI_GROUP,
    }
}

#[cached]
pub async fn get_comment(id: u32) -> Vec<NHentaiComment> {
    let response =
        get::<Vec<NHentaiComment>>(format!("https://nhentai.net/api/gallery/{}/comments", id));

    if let Ok(comments) = response.await {
        comments
    } else {
        vec![]
    }
}

pub async fn get_comment_range(
    id: u32,
    from: Option<u32>,
    to: Option<u32>,
    batch: Option<u32>,
    batch_by: Option<u32>,
    order_by: Option<NhqlCommentOrder>,
) -> Vec<NHentaiComment> {
    let mut comments = get_comment(id).await;

    if order_by.unwrap_or(NhqlCommentOrder::Newest) == NhqlCommentOrder::Oldest {
        comments.sort_by(|a, b| a.post_date.cmp(&b.post_date));
    }

    if let Some(batch) = batch {
        let mut result = vec![];
        let batch_by = batch_by.unwrap_or(25);

        if batch <= 0 {
            return vec![];
        }

        let batch_from = (batch - 1) * batch_by;
        let batch_to = batch * batch_by;

        for index in (batch_from)..(batch_to) {
            if (index as usize) >= comments.len() {
                break;
            }

            result.push(comments[index as usize].clone());
        }

        return result;
    }

    let mut result = vec![];
    let from = from.unwrap_or(0);
    let to = to.unwrap_or(comments.len() as u32) + 1;

    for index in (from)..(to) {
        if (index as usize) >= comments.len() {
            break;
        }

        result.push(comments[index as usize].clone());
    }

    result
}

#[cached]
pub async fn get_related(id: u32) -> Vec<NHentai> {
    let response = get::<NHentaiRelated>(format!("https://nhentai.net/api/gallery/{}/related", id));

    if let Ok(related) = response.await {
        related.result
    } else {
        vec![]
    }
}

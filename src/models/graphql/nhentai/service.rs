use super::{
    constant::*, 
    model::*, 
    super::nhql::model::{ NhqlCommentOrder, NhqlChannel }
};
use crate::{services::request::get};

use async_graphql::InputType;
use cached::{ proc_macro::cached, TimedCache };

use futures::{stream, StreamExt};
use tokio::time::{sleep, Duration};

const PARALLEL_REQUESTS: usize = 25;

pub async fn get_nhentais_by_id(id: Vec<u32>) -> Vec<NHentai> {
    let limit = id.len();
    let (tx, mut rx) = tokio::sync::mpsc::channel::<NHentai>(limit);

    let responses = stream::iter(id)
        .map(|id| {
            tokio::spawn(async move {
                sleep(Duration::from_millis(100)).await;
                get_nhentai_by_id(id, NhqlChannel::Hifumin).await
            })
        })
        .buffered(PARALLEL_REQUESTS);

    responses
        .for_each(|res| async {
            match tx.send(res.unwrap_or(EMPTY_NHENTAI_DATA)).await {
                Ok(_a) => {},
                Err(_e) => {}
            }
        })
        .await;

    let mut hentais: Vec<NHentai> = vec![];
    while let Some(nhentai) = rx.recv().await {
        hentais.push(nhentai);

        if hentais.len() >= limit {
            break
        }
    }

    hentais
}

#[cached(
    type = "TimedCache<String, Option<InternalNHentai>>",
    create = "{ TimedCache::with_lifespan(3600 * 3) }",
    convert = r#"{ format!("{}-{}", channel.to_value(), id) }"#
)]
pub async fn internal_get_nhentai_by_id(id: u32, channel: NhqlChannel) -> Option<InternalNHentai> {
    // ? It would take a very long time until nhentai get more id than 10M
    if id > 10_000_000 {
        return None
    }

    let endpoint = match channel {
        NhqlChannel::Hifumin => format!("https://raw.githubusercontent.com/saltyaom-engine/hifumin-mirror/generated/{}.json", id),
        NhqlChannel::HifuminFirst => format!("https://raw.githubusercontent.com/saltyaom-engine/hifumin-mirror/generated/{}.json", id),
        NhqlChannel::Nhentai => format!("https://nhentai.net/api/gallery/{}", id),
    };

    if let Ok(nhentai) = get::<InternalNHentai>(endpoint).await {
        return Some(nhentai)
    }

    if channel != NhqlChannel::Hifumin {
        return None
    }

    if let Ok(nhentai) = get::<InternalNHentai>(
        format!("https://nhentai.net/api/gallery/{}", id
    )).await {
        return Some(nhentai)
    }

    None
}

pub async fn get_nhentai_by_id(id: u32, channel: NhqlChannel) -> NHentai {
    if let Some(nhentai) = internal_get_nhentai_by_id(id, channel).await {
        NHentai {
            id: Some(id),
            title: nhentai.title,
            media_id: nhentai.media_id,
            images: nhentai.images,
            scanlator: nhentai.scanlator,
            upload_date: nhentai.upload_date,
            tags: nhentai.tags,
            num_pages: nhentai.num_pages,
            num_favorites: nhentai.num_favorites,
            channel: channel,
        }
    } else {
        match channel {
            NhqlChannel::Hifumin => EMPTY_NHENTAI_HIFUMIN_DATA,
            _ => EMPTY_NHENTAI_DATA
        }
    }
}

#[cached(
    type = "TimedCache<String, NHentaiGroup>",
    create = "{ TimedCache::with_lifespan(3 * 3600) }",
    convert = r#"{ format!("{}{}{}{}{}{}{}", channel.to_value(), search.to_lowercase(), page, includes.join(""), excludes.join(""), tags.join(""), artists.join("") ) }"#
)]
pub async fn search_nhentai(
    channel: NhqlChannel,
    search: String,
    page: u16,
    includes: Vec<String>,
    excludes: Vec<String>,
    tags: Vec<String>,
    artists: Vec<String>
) -> NHentaiGroup {
    if channel == NhqlChannel::Hifumin || channel == NhqlChannel::HifuminFirst {
        let hentais = match get::<Vec<u32>>(
            format!(
                "https://search.hifumin.app/search/{}/{}",
                search, page
            )
        ).await {
            Ok(ids) => get_nhentais_by_id(ids).await ,
            Err(_error) => vec![]
        };

        if channel != NhqlChannel::HifuminFirst && hentais.len() > 0 {
            return NHentaiGroup {
                num_pages: None,
                per_page: Some(25),
                result: hentais.into_iter().map(|hentai| NHentai {
                    id: hentai.id,
                    title: hentai.title,
                    media_id: hentai.media_id,
                    images: hentai.images,
                    scanlator: hentai.scanlator,
                    upload_date: hentai.upload_date,
                    tags: hentai.tags,
                    num_pages: hentai.num_pages,
                    num_favorites: hentai.num_favorites,
                    channel,
                }).collect(),
            }
        }
    }

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

    match get::<InternalNHentaiGroup>(
        format!(
            "https://nhentai.net/api/galleries/search?query={}&page={}",
            query, page
        )
    ).await {
        Ok(nhentai) => NHentaiGroup {
            num_pages: nhentai.num_pages,
            per_page: nhentai.per_page,
            result: nhentai.result.into_iter().map(|hentai| NHentai {
                id: hentai.id,
                title: hentai.title,
                media_id: hentai.media_id,
                images: hentai.images,
                scanlator: hentai.scanlator,
                upload_date: hentai.upload_date,
                tags: hentai.tags,
                num_pages: hentai.num_pages,
                num_favorites: hentai.num_favorites,
                channel,
            }).collect(),
        },
        Err(_error) => EMPTY_NHENTAI_GROUP
    }
}


#[cached(
    type = "TimedCache<String, Vec<NHentaiComment>>",
    create = "{ TimedCache::with_lifespan(3 * 3600) }",
    convert = r#"{ format!("{}-{}", channel.to_value(), id) }"#
)]
pub async fn get_comment(id: u32, channel: NhqlChannel) -> Vec<NHentaiComment> {
    // ? It would take a very long time until nhentai get more id than 10M
    if id > 10_000_000 {
        return vec![]
    }

    let endpoint = match channel {
        NhqlChannel::Hifumin => format!("https://raw.githubusercontent.com/saltyaom-engine/hifumin-comment-mirror/generated/{}.json", id),
        NhqlChannel::HifuminFirst => format!("https://raw.githubusercontent.com/saltyaom-engine/hifumin-comment-mirror/generated/{}.json", id),
        NhqlChannel::Nhentai => format!("https://nhentai.net/api/gallery/{}/comments", id)
    };

    if let Ok(comments) = get::<Vec<NHentaiComment>>(endpoint).await {
        return comments
    }

    if channel != NhqlChannel::Hifumin {
        return vec![]
    }

    if let Ok(comments) = get::<Vec<NHentaiComment>>(
        format!("https://nhentai.net/api/gallery/{}/comments", id)
    ).await {
        return comments
    }

    vec![]
}

pub async fn get_comment_range(
    id: u32,
    from: Option<u32>,
    to: Option<u32>,
    batch: Option<u32>,
    batch_by: Option<u32>,
    order_by: Option<NhqlCommentOrder>,
    channel: NhqlChannel
) -> Vec<NHentaiComment> {
    let mut comments = get_comment(id, channel).await;

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

#[cached(
    type = "TimedCache<u32, Vec<NHentai>>",
    create = "{ TimedCache::with_lifespan(6 * 3600) }",
    convert = r#"{ id }"#
)]
pub async fn get_related(id: u32) -> Vec<NHentai> {
    // ? It would take a very long time until nhentai get more id than 1M
    if id > 750_000 {
        return vec![]
    }

    let response = get::<NHentaiRelated>(format!("https://nhentai.net/api/gallery/{}/related", id));

    if let Ok(related) = response.await {
        related.result
    } else {
        vec![]
    }
}

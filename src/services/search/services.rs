use reqwest;
use cached::{proc_macro::cached, TimedCache};
use std::{collections::HashMap};

use meilisearch_sdk::{
    client::Client,
    indexes::Index,
    search::{SearchResult, Query},
};

use super::models::{ HentaiSearch, Status };

lazy_static! {
    pub static ref SEARCH_ENGINE: Index = Client::new("http://localhost:7700", "masterKey").index("hentai");
}

pub async fn wait_for_search_engine() {
    let client = reqwest::Client::new();

    // interval 1s
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));

    loop {
        interval.tick().await;

        match client.get("http://localhost:7700/health").send().await {
            Ok(response) => {
                match response.json::<Status>().await {
                    Ok(res) => {
                        if res.status == "available" {
                            break;
                        }        
                    },
                    Err(_) => {
                        continue;
                    }
                }
            }
            Err(_) => {
                continue;
            }
        }
    }

    // Boot up search instance
    search("yuri".to_owned(), 1).await;
    search("glasses".to_owned(), 1).await;
}

lazy_static! {
    static ref FILTERS: HashMap<String, &'static str> = HashMap::from([
        ("yuri".to_owned(), r#"(tags != "yaoi") AND (tags != "yuri or ice") AND (tags != "yuuri") AND (tags != "males only")"#)
    ]);
}

#[cached(
    type = "TimedCache<String, Vec<u32>>",
    create = "{ TimedCache::with_lifespan(6 * 3600) }",
    convert = r#"{ format!("{}{}",keyword, batch) }"#
)]
pub async fn search<'a>(keyword: String, batch: u16) -> Vec<u32> {
    // Limitation of Meilisearch
    if batch > 40 {         
        return vec![]
    }

    let offset = (batch - 1) as usize * 25;

    let query = if let Some(filter) = FILTERS.get(&keyword) {
        Query::new(&SEARCH_ENGINE)
            .with_query(&keyword)
            .with_limit(25)
            .with_offset(offset)
            .with_filter(filter)
            .build()
    } else {
        Query::new(&SEARCH_ENGINE)
            .with_query(&keyword)
            .with_limit(25)
            .with_offset(offset)
            .build()
    };

    match SEARCH_ENGINE.execute_query(&query).await {
        Ok(results) => results
            .hits
            .into_iter()
            .map(|hit: SearchResult<HentaiSearch>| hit.result.id)
            .collect(),
        Err(_) => vec![]
    }
}

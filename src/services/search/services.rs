use reqwest;
use std::collections::HashMap;

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
    search("yuri".to_owned(), 1, &vec![]).await;
    search("glasses".to_owned(), 1, &vec![]).await;
}

lazy_static! {
    static ref FILTERS: HashMap<String, &'static str> = HashMap::from([
        ("yuri".to_owned(), r#"(tags != "yaoi") AND (tags != "yuri or ice") AND (tags != "yuuri") AND (tags != "males only")"#)
    ]);
}

pub async fn search<'a>(keyword: String, batch: u16, excludes: &Vec<String>) -> Vec<u32> {
    // Limitation of Meilisearch
    if batch > 40 {         
        return vec![]
    }

    let to_excludes = excludes.iter().map(|tag| format!("(tags != \"{}\")", tag)).collect::<Vec<String>>().join(" AND ");

    let offset = (batch - 1) as usize * 25;

    let mut unioned: String = String::new();

    let query = if let Some(filter) = FILTERS.get(&keyword) {
        if to_excludes != "" {
            unioned = format!("{} AND {}", filter, to_excludes);

            Query::new(&SEARCH_ENGINE)
                .with_query(&keyword)
                .with_limit(25)
                .with_offset(offset)
                .with_filter(&unioned)
                .build()
        } else {
            Query::new(&SEARCH_ENGINE)
                .with_query(&keyword)
                .with_limit(25)
                .with_offset(offset)
                .with_filter(filter)
                .build()
        }
    } else if to_excludes != "" {
        Query::new(&SEARCH_ENGINE)
            .with_query(&keyword)
            .with_limit(25)
            .with_offset(offset)
            .with_filter(&to_excludes)
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

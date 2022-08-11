use reqwest;
use std::{collections::{HashMap, hash_map::DefaultHasher}, hash::{Hash, Hasher}};

use meilisearch_sdk::{
    client::Client,
    indexes::Index,
    search::{SearchResult, Query},
};

use crate::models::graphql::nhql::model::NhqlChannel;
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
    search(SearchOption::keyword("yuri")).await;
    search(SearchOption::keyword("glasses")).await;
}

lazy_static! {
    static ref FILTERS: HashMap<String, &'static str> = HashMap::from([
        ("yuri".to_owned(), r#"(tags != "yaoi") OR (tags != "yuri or ice") OR (tags != "yuuri") OR (tags != "males only")"#)
    ]);
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct SearchOption {
    pub keyword: String,
    pub channel: NhqlChannel,
    pub batch: u16,
    pub includes: Vec<String>,
    pub excludes: Vec<String>
}

impl SearchOption {
    pub fn new(keyword: impl Into<String>, batch: u16, includes: Vec<String>, excludes: Vec<String>) -> Self {
        Self {
            keyword: keyword.into(),
            batch,
            channel: NhqlChannel::HifuminFirst,
            includes,
            excludes,
        }
    }

    pub fn keyword(keyword: impl Into<String>) -> Self {
        Self {
            keyword: keyword.into(),
            batch: 1,
            channel: NhqlChannel::HifuminFirst,
            includes: vec![],
            excludes: vec![],
        }
    }

    pub fn with_batch(keyword: impl Into<String>, batch: u16) -> Self {
        Self {
            keyword: keyword.into(),
            batch,
            channel: NhqlChannel::HifuminFirst,
            includes: vec![],
            excludes: vec![],
        }
    }

    pub fn hash_value(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl Default for SearchOption {
    fn default() -> Self {
        Self { 
            keyword: "".to_owned(), 
            batch: 1,
            channel: NhqlChannel::HifuminFirst,
            includes: vec![], 
            excludes: vec![] 
        }
    }
}

pub async fn search<'a>(search: SearchOption) -> Vec<u32> {
    let SearchOption { keyword, batch, includes, excludes, .. } = search;

    // Limitation of Meilisearch
    if batch > 40 {         
        return vec![]
    }

    let offset = (batch - 1) as usize * 25;

    let to_includes = includes.iter().map(|tag| format!("(tags = \"{}\")", tag)).collect::<Vec<String>>().join(" AND ");
    let to_excludes = excludes.iter().map(|tag| format!("(tags != \"{}\")", tag)).collect::<Vec<String>>().join(" OR ");

    let mut query = Query::new(&SEARCH_ENGINE)
        .with_query(&keyword)
        .with_limit(25)
        .with_offset(offset)
        .build();

    if let Some(filter) = FILTERS.get(&keyword) {
        query = query.with_filter(filter).build()
    }

    if to_includes != "" {
        query = query.with_filter(&to_includes).build()
    }

    if to_excludes != "" {
        query = query.with_filter(&to_excludes).build()
    }

    match SEARCH_ENGINE.execute_query(&query).await {
        Ok(results) => results
            .hits
            .into_iter()
            .map(|hit: SearchResult<HentaiSearch>| hit.result.id)
            .collect(),
        Err(_) => vec![]
    }
}

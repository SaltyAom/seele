use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HentaiSearch {
    pub id: u32
}

#[derive(Deserialize)]
pub struct Status {
    pub status: String
}

#[derive(Serialize, Deserialize)]
pub struct TagIndex {
    pub total: u32,
    pub ids: Vec<u32>
}

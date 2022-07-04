use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HentaiSearch {
    pub id: u32
}

#[derive(Deserialize)]
pub struct Status {
    pub status: String
}
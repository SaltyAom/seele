use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HentaiSearch {
    pub id: u32
}

#[derive(Deserialize, Debug)]
pub struct Status {
    pub status: String
}
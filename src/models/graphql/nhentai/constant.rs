use super::model::*;

pub const EMPTY_NHENTAI_DATA: NHentai = NHentai {
    id: None,
    title: NHentaiTitle {
        english: None,
        japanese: None,
        pretty: None
    },
    media_id: None,
    images: NHentaiImages {
        pages: vec![],
        cover: NHentaiPage {
            t: None, 
            w: None, 
            h: None
        },
        thumbnail: NHentaiPage { 
            t: None, 
            w: None, 
            h: None 
        }
    },
    scanlator: None,
    upload_date: None,
    tags: vec![],
    num_pages: None,
    num_favorites: None
};

pub const EMPTY_NHENTAI_GROUP: NHentaiGroup = NHentaiGroup {
    result: vec![],
    num_pages: None,
    per_page: None    
};
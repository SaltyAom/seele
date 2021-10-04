use actix_web::{ get, web::ServiceConfig };
use actix_files::NamedFile;

#[get("/og.jpg")]
pub async fn og_image() -> NamedFile {
    NamedFile::open("public/og.jpg").unwrap()
}

pub fn use_decoration(config: &mut ServiceConfig) {
    config
        .service(og_image);
}
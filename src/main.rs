#[macro_use]
extern crate lazy_static;

mod models;
mod controllers;
mod services;

use actix_web::{ HttpServer, App, web::Data };
use actix_cors::Cors;

use controllers::{ use_decoration, use_graphql };
use models::graphql::create_schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = create_schema();
    HttpServer::new(move ||
        App::new()
            .wrap(Cors::permissive().allow_any_origin())
            .app_data(Data::new(schema.clone()))
            .configure(use_graphql)
            .configure(use_decoration)
    )
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

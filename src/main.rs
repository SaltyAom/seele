#[macro_use]
extern crate lazy_static;

mod models;
mod controllers;
mod services;

use actix_web::{ HttpServer, App, web::Data };
use actix_cors::Cors;

use controllers::{ use_decoration, use_graphql };
use models::graphql::create_schema;
use services::search::wait_for_search_engine;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🦋 Seele is starting...");
    
    wait_for_search_engine().await;
    
    let schema = create_schema();

    println!("🦋 Seele is running...");

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

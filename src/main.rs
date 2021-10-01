mod models;
mod controllers;
mod services;

use actix_web::{ HttpServer, App };

use controllers::use_graphql;
use models::graphql::create_schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = create_schema();

    HttpServer::new(move ||
        App::new()
            .data(schema.clone())
            .configure(use_graphql)
    )
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

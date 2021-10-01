use actix_web::{web::{ Data, ServiceConfig }, get, post, HttpResponse, Result};
use async_graphql_actix_web::{ Response, Request };
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};

use crate::models::AppSchema;

#[post("/graphql")]
pub async fn graphql_executor(
    schema: Data<AppSchema>,
    request: Request
) -> Response {
    schema.execute(request.into_inner()).await.into()
}

#[get("/graphql")]
pub async fn playground() -> Result<HttpResponse> {
    Ok(
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(playground_source(
                GraphQLPlaygroundConfig::new("/graphql"),
            )
        )
    )
}

pub fn use_graphql(config: &mut ServiceConfig) {
    config
        .service(graphql_executor)
        .service(playground);
}
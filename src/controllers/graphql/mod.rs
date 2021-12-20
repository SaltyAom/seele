use actix_web::{web::{ Data, ServiceConfig }, get, post, HttpResponse, Result};
use async_graphql_actix_web::{ GraphQLRequest, GraphQLResponse };
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};

use crate::models::AppSchema;

#[post("/v1/graphql")]
pub async fn graphql_executor(
    schema: Data<AppSchema>,
    request: GraphQLRequest
) -> GraphQLResponse {
    schema.execute(request.into_inner()).await.into()
}

#[get("/v1/graphql")]
pub async fn playground() -> Result<HttpResponse> {
    Ok(
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(playground_source(
                GraphQLPlaygroundConfig::new("/v1/graphql"),
            )
        )
    )
}

pub fn use_graphql(config: &mut ServiceConfig) {
    config
        .service(graphql_executor)
        .service(playground);
}
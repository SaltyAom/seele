use actix_web::{web::{ Data, ServiceConfig }, get, post, HttpResponse, Result};
use async_graphql_actix_web::{ GraphQLRequest, GraphQLResponse };
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};

use crate::models::AppSchema;

#[post("/graphql")]
pub async fn graphql_executor(
    schema: Data<AppSchema>,
    request: GraphQLRequest
) -> GraphQLResponse {
    schema.execute(request.into_inner()).await.into()
}

#[get("/")]
pub async fn landing() -> Result<HttpResponse> {
    Ok(
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(playground_source(
                GraphQLPlaygroundConfig::new("/v1/graphql"),
            )
        )
    )
}

#[get("/graphql")]
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

#[post("/v1/graphql")]
pub async fn v1_graphql_executor(
    schema: Data<AppSchema>,
    request: GraphQLRequest
) -> GraphQLResponse {
    let t1 = std::time::Instant::now();
    let result = schema.execute(request.into_inner()).await.into();
    println!("GQL: {:?}\n", t1.elapsed().as_secs_f64());

    result
}

#[get("/v1/graphql")]
pub async fn v1_playground() -> Result<HttpResponse> {
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
        .service(landing)
        .service(graphql_executor)
        .service(playground)
        .service(v1_graphql_executor)
        .service(v1_playground);
}

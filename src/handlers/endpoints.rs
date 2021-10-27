use actix_web::{web, HttpResponse, HttpRequest};
use async_graphql::http::GraphQLPlaygroundConfig;

//use juniper::http::{GraphQLRequest};
//use juniper::http::playground::playground_source;
use async_graphql_actix_web::{Request, Response};

use crate::models;
use crate::graphql::{AppSchema};


pub async fn playground_handler() -> HttpResponse {
    let html = async_graphql::http::playground_source(
        GraphQLPlaygroundConfig::new("/graphql"));
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn graphql(
    //pg_pool: web::Data<PostgresPool>,
    schema: web::Data<AppSchema>,
    http_request: HttpRequest,
    req: Request,
) -> Response {
    
    let mut query = req.into_inner();

    let maybe_role_id = models::get_role_and_id(http_request);
    if let Some((role, uuid)) = maybe_role_id {
        query = query.data(role);
        query = query.data(uuid);
    }

    schema.execute(query).await.into()
}
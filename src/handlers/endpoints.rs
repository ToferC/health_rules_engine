use actix_web::{web, HttpResponse, HttpRequest};
use actix_web_httpauth::extractors::AuthExtractorConfig;
use async_graphql::http::GraphQLPlaygroundConfig;

//use juniper::http::{GraphQLRequest};
//use juniper::http::playground::playground_source;
use std::sync::Arc;
use async_graphql::*;
use async_graphql_actix_web::{Request, Response};

use crate::{database::PostgresPool};
use crate::models;
use crate::graphql::{Query, Mutation, AppSchema};


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

    let maybe_role = models::get_role(http_request);
    if let Some(role) = maybe_role {
        query = query.data(role);
    }

    schema.execute(query).await.into()
}
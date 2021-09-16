use diesel::prelude::*;
use diesel::serialize::Result;
use diesel::{QueryDsl, BelongingToDsl};
use actix_web::{web, get, post, HttpResponse, HttpRequest, Responder,
    Error,    
};

use juniper::http::graphiql::graphiql_source;
use juniper::http::{GraphQLRequest};
use juniper::http::playground::playground_source;
use std::sync::Arc;

use tera::Context;

use crate::{AppData, GraphQLContext, schema};
use crate::errors::error_handler::CustomError;
use crate::database::PostgresPool;
use crate::graphql::{Schema, create_context};


pub async fn playground_handler() -> HttpResponse {
    let html = playground_source("/graphql", Some("/subscriptions"));
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn graphql(
    pg_pool: web::Data<PostgresPool>,
    schema: web::Data<Arc<Schema>>,
    data_query: web::Json<GraphQLRequest>,
) -> HttpResponse {
    let ctx = create_context(pg_pool.as_ref().to_owned());

    let res = data_query.execute(&schema, &ctx).await;

    HttpResponse::Ok()
        .content_type("application/json")
        .json(res)
}
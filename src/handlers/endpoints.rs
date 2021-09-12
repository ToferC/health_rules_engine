use diesel::prelude::*;
use diesel::{QueryDsl, BelongingToDsl};
use actix_web::{web, get, post, HttpResponse, HttpRequest, Responder,
    Error,    
};

use juniper::http::graphiql::graphiql_source;
use juniper::http::{GraphQLRequest};
use juniper::http::playground::playground_source;

use tera::Context;

use crate::{AppData, GraphQLContext};
use crate::errors::error_handler::CustomError;
use crate::database::PostgresPool;


#[get("/playground")]
pub async fn playground_handler() -> HttpResponse {
    let html = playground_source("/graphql", Some("/subscriptions"));
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[post("/graphql")]
async fn graphql(
    pool: web::Data<PostgresPool>,
    schema: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let ctx = GraphQLContext {
        pool: pool.get_ref().to_owned(),
    };

    let res = web::block(move || {
        let res = data.execute(&schema, &ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await
    .map_err(Error::from)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(res))
}
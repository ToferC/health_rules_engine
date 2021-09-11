use actix_web::{web, get, HttpResponse, HttpRequest, Responder};
//use actix_identity::Identity;
use diesel::prelude::*;
use diesel::{QueryDsl, BelongingToDsl};

use juniper::http::graphiql::graphiql_source;
use juniper::http::{GraphQLRequest};
use juniper::http::playground::playground_source;

use tera::Context;

use crate::AppData;

#[get("/")]
pub async fn index(data: web::Data<AppData>, _req:HttpRequest) -> impl Responder {
    let ctx = Context::new(); 
    let rendered = data.tmpl.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/{lang}/api")]
pub async fn api_base(
    data: web::Data<AppData>,
    _lang: web::Path<String>,
    _req: HttpRequest,
    // id: Identity,
) -> impl Responder {

    let ctx = Context::new(); 
    let rendered = data.tmpl.render("api_base.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/playground")]
pub async fn playground_handler() -> HttpResponse {
    let html = playground_source("/graphql", Some("/subscriptions"));
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}



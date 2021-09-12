#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate juniper;

use std::env;
use actix_web::web::Data;
use actix_web::{App, HttpServer, middleware};
use tera::{Tera};
use tera_text_filters::snake_case;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod models;
mod handlers;
mod errors;
mod schema;
mod database;
mod graphql;

use crate::database::{POOL, PostgresPool};

pub struct AppData {
    tmpl: Tera
}

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct GraphQLContext {
    pub pool: PostgresPool,
}

impl juniper::Context for GraphQLContext {}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    database::init();

    let environment = env::var("ENVIRONMENT");

    let environment = match environment {
        Ok(v) => v,
        Err(_) => String::from("test"),
    };

    let (host, port) = if environment == "production" {
        (env::var("HOST").unwrap(), env::var("PORT").unwrap())
    } else {
        (String::from("127.0.0.1"), String::from("8088"))
    };

    println!("{}", env!("CARGO_MANIFEST_DIR"));

    println!("Serving on: {}:{}", &host, &port);

    HttpServer::new(move || {
        
        let mut tera = Tera::new(
            "templates/**/*").unwrap();

        tera.register_filter("snake_case", snake_case);
        tera.full_reload().expect("Error running auto reload with Tera");

        App::new()
            .data(POOL.clone())
            .data(AppData {tmpl: tera})
            .wrap(middleware::Logger::default())
            .configure(handlers::init_routes)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

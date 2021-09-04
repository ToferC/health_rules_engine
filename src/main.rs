extern crate juniper;
extern crate diesel;

use std::env;
use actix_web::{App, HttpServer, web, middleware, web::Json};
use tera::{Tera};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod models;

use models::*;

struct AppData {
    tmpl: Tera
}

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct Database {
    pub data: String
}

impl juniper::Context for Database {}

impl Database {
    pub fn new() -> Self {
        Database { data: String::from("Insert Data Here") }
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let environment = env::var("ENVIRONMENT");

    let environment = match environment {
        Ok(v) => v,
        Err(_) => String::from("test"),
    };

    let (host, port) = if environment == "production" {
        (env::var("HOST").unwrap(), env::var("PORT").unwrap())
    } else {
        (String::from("127.0.0.1"), String::from("8080"))
    };

    println!("{}", env!("CARGO_MANIFEST_DIR"));

    let ctx = Database::new();

    HttpServer::new(move || {
        
        let tera = Tera::new(
            "templates/**/*").unwrap();

        App::new()
            .data(ctx.clone())
            .data(AppData {tmpl: tera})
            .wrap(middleware::Logger::default())
            .service(handlers::index)
            .service(web::resource("/playground").route(web::get().to(handlers::playground_handler)))

    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

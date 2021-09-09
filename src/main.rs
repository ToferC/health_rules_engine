extern crate juniper;
extern crate diesel;

use std::env;
use actix_web::{App, HttpServer, middleware};
use tera::{Tera};
use tera_text_filters::snake_case;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod models;
mod handlers;

pub struct AppData {
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
        
        let mut tera = Tera::new(
            "templates/**/*").unwrap();

        tera.register_filter("snake_case", snake_case);
        tera.full_reload().expect("Error running auto reload with Tera");

        App::new()
            .data(ctx.clone())
            .data(AppData {tmpl: tera})
            .wrap(middleware::Logger::default())
            .configure(handlers::init_routes)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

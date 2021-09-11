#[macro_use]
extern crate diesel;
extern crate juniper;

use std::env;
use actix_web::{App, HttpServer, middleware};
use tera::{Tera};
use tera_text_filters::snake_case;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod models;
mod handlers;
//mod errors;
mod schema;

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

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

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

    let ctx = Database::new();

    HttpServer::new(move || {
        
        let mut tera = Tera::new(
            "templates/**/*").unwrap();

        tera.register_filter("snake_case", snake_case);
        tera.full_reload().expect("Error running auto reload with Tera");

        App::new()
            .data(pool.clone())
            .data(ctx.clone())
            .data(AppData {tmpl: tera})
            .wrap(middleware::Logger::default())
            .configure(handlers::init_routes)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

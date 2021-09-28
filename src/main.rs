use std::env;
use actix_web::{App, HttpServer, middleware};
use tera::{Tera};
use tera_text_filters::snake_case;

use health_rules_engine::database::{self, POOL};
use health_rules_engine::graphql::{create_schema};
use health_rules_engine::AppData;
use health_rules_engine::handlers;

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

    // Create Schema
    let schema = std::sync::Arc::new(create_schema());

    HttpServer::new(move || {
        
        let mut tera = Tera::new(
            "templates/**/*").unwrap();

        tera.register_filter("snake_case", snake_case);
        tera.full_reload().expect("Error running auto reload with Tera");

        App::new()
            .data(POOL.clone())
            .data(schema.clone())
            .data(AppData {tmpl: tera})
            .wrap(middleware::Logger::default())
            .configure(handlers::init_routes)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

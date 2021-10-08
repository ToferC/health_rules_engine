use std::env;
use actix_web::{App, HttpServer, middleware};
use tera::{Tera};
use tera_text_filters::snake_case;
use actix_identity::{CookieIdentityPolicy, IdentityService};

use health_rules_engine::database::{self, POOL};
use health_rules_engine::graphql::{create_schema_with_context};
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

    let SECRET_KEY = env::var("SECRET_KEY").expect("Unable to find secret key");

    let (host, port) = if environment == "production" {
        (env::var("HOST").unwrap(), env::var("PORT").unwrap())
    } else {
        (String::from("127.0.0.1"), String::from("8088"))
    };

    let domain = host.clone();

    println!("{}", env!("CARGO_MANIFEST_DIR"));

    println!("Serving on: {}:{}", &host, &port);

    // Create Schema
    let schema = create_schema_with_context(POOL.clone());

    HttpServer::new(move || {
        
        let mut tera = Tera::new(
            "templates/**/*").unwrap();

        tera.register_filter("snake_case", snake_case);
        tera.full_reload().expect("Error running auto reload with Tera");

        App::new()
            //.data(POOL.clone())
            .data(AppData {tmpl: tera})
            .data(schema.clone())
            .configure(handlers::init_routes)
            .wrap(middleware::Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(SECRET_KEY.as_bytes())
                    .name("auth")
                    .path("/graphiql")
                    .domain(domain.clone())
                    .max_age(86400)
                    .secure(false)
            ))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

use std::env;
use actix_web::{web, App, HttpServer, middleware};
use diesel::IntoSql;
use tera::{Tera};
use tera_text_filters::snake_case;

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

    let _secret_key = env::var("SECRET_KEY").expect("Unable to find secret key");

    let (host, port) = if environment == "production" {
        (env::var("HOST").unwrap(), env::var("PORT").unwrap())
    } else {
        (String::from("0.0.0.0"), String::from("8080"))
    };

    let _domain = host.clone();

    println!("{}", env!("CARGO_MANIFEST_DIR"));

    println!("Serving on: {}:{}", &host, &port);

    // Create Schema
    let schema = web::Data::new(create_schema_with_context(POOL.clone()));

    
    HttpServer::new(move || {
        
        let mut tera = Tera::new(
            "templates/**/*").unwrap();
            
        tera.register_filter("snake_case", snake_case);
        tera.full_reload().expect("Error running auto reload with Tera");
        
        let app_data = web::Data::new(AppData {tmpl: tera});

        App::new()
            //.data(POOL.clone())
            .configure(handlers::configure_services)
            .app_data(schema.clone())
            .app_data(app_data)
            .wrap(middleware::Logger::default())
    })
    .bind((format!("{}", host).as_str(), 8080))?
    .run()
    .await
}

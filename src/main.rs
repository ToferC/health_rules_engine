#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate juniper;

use std::collections::HashMap;
use std::env;
use actix_web::web::Data;
use actix_web::{App, HttpServer, middleware};
use juniper::FieldResult;
use models::{Place, Vaccine, Country};
use tera::{Tera};
use tera_text_filters::snake_case;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use uuid::Uuid;

mod models;
mod handlers;
mod errors;
mod schema;
mod database;
mod graphql;

use crate::database::{POOL, PostgresPool};
use crate::graphql::{Schema, create_schema};

pub struct AppData {
    tmpl: Tera
}

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct GraphQLContext {
    pub pool: PostgresPool,
    // Standard query items here so we don't need to go to db
    pub countries: HashMap<Uuid, models::Country>,
    pub places: HashMap<Uuid, models::Place>,
    pub vaccines: HashMap<Uuid, models::Vaccine>,

}

impl juniper::Context for GraphQLContext {}

impl GraphQLContext {
    pub fn get_place_by_id(&self, id: Uuid) -> FieldResult<Place> {
        let place = self.places
            .get(&id)
            .expect("Unable to retrieve Place");

        Ok(place.clone())
    }

    pub fn get_country_by_id(&self, id: Uuid) -> FieldResult<Country> {
        let country = self.countries
            .get(&id)
            .expect("Unable to retrieve Country");

        Ok(country.clone())
    }

    pub fn get_vaccine_by_id(&self, id: Uuid) -> FieldResult<Vaccine> {
        let vaccine = self.vaccines
            .get(&id)
            .expect("Unable to retrieve Vaccine");

        Ok(vaccine.clone())
    }
}

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

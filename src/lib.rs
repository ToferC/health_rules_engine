#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate juniper;

use std::collections::HashMap;
use juniper::FieldResult;
use models::{Place, Vaccine, Country};
use tera::{Tera};

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use uuid::Uuid;

pub mod models;
pub mod handlers;
pub mod errors;
pub mod schema;
pub mod database;
pub mod graphql;

use crate::database::{PostgresPool};

pub struct AppData {
    pub tmpl: Tera
}

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

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
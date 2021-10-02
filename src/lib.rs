#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate juniper;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use juniper::FieldResult;
use models::{Country, NewCountry, Place, Vaccine};
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

const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub struct AppData {
    pub tmpl: Tera
}

#[derive(Clone)]
pub struct GraphQLContext {
    pub pool: PostgresPool,
    // Standard query items here so we don't need to go to db
    pub countries: Arc<Mutex<HashMap<Uuid, models::Country>>>,
    pub places: Arc<Mutex<HashMap<Uuid, models::Place>>>,
    pub vaccines: HashMap<Uuid, models::Vaccine>,
}

impl juniper::Context for GraphQLContext {}

impl GraphQLContext {
    pub fn get_place_by_id(&self, id: Uuid) -> FieldResult<Place> {

        let places = self.places.lock().unwrap();

        let place = places
            .get(&id)
            .expect("Unable to retrieve Place");

        Ok(place.clone())
    }

    // Change back to get_or_create_place_by_name_and_country_id
    pub fn get_or_create_place_by_name_and_country_id(&self, name: String, country_id: Uuid) -> FieldResult<Place> {

        let mut places = self.places.lock().unwrap();

        let res = places.iter()
            .find_map(
                |(_key, val)| 
                if val.name == name && val.country_id == country_id { 
                    Some(val.clone()) 
                } else { None });
        
        let place = match res {
            Some(p) => p,
            None => {
                let p = models::NewPlace::new(name, country_id);
                let place = models::Place::create(
                    &self.pool.get().expect("Unable to connect to db"), 
                    &p)?;
                
                places.insert(place.id, place.clone());
                drop(places);
                place
            }
        };

        Ok(place.clone())
    }

    pub fn get_country_by_id(&self, id: Uuid) -> FieldResult<Country> {

        let countries = self.countries.lock().unwrap();

        let country = countries
            .get(&id)
            .expect("Unable to retrieve Country");

        Ok(country.clone())
    }

    pub fn get_or_create_country_by_name(&self, country_name: String) -> FieldResult<Country> {

        let mut countries = self.countries.lock().unwrap();

        let res = countries.iter()
            .find_map(|(_key, val)| if val.country_name == country_name { Some(val) } else { None });

        let country = match res {
            Some(c) => c.clone(),

            // None should *rarely* happen
            None => {
                let c = NewCountry::new(country_name, 0.03);

                // Insert country into DB
                let country = Country::create(
                    &self.pool.get().expect("Unable to connec to db"), 
                    &c)?;
                
                // Insert into Hashmap cache
                countries.insert(country.id, country.clone());
                drop(countries);
                
                country
            }
        };
        
        Ok(country.clone())
    }

    pub fn get_vaccine_by_id(&self, id: Uuid) -> FieldResult<Vaccine> {
        let vaccine = self.vaccines
            .get(&id)
            .expect("Unable to retrieve Vaccine");

        Ok(vaccine.clone())
    }

    pub fn get_vaccine_by_name(&self, name: String) -> FieldResult<Vaccine> {
        let res = self.vaccines.iter()
            .find_map(|(_key, val)| if val.vaccine_name == name { Some(val) } else { None })
            .expect("Unable to find vaccine");

        Ok(res.clone())
    }
}

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
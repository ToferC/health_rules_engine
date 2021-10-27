#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate async_graphql;

#[macro_use]
extern crate shrinkwraprs;

#[macro_use]
extern crate strum_macros;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use async_graphql::*;
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
pub mod common_utils;

use crate::graphql::{get_connection_from_context};
use crate::models::{SlimUser, User, verify_password};

const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub struct AppData {
    pub tmpl: Tera
}

pub fn get_place_by_id(context: &Context<'_>, id: Uuid) -> FieldResult<Place> {

let places = context.data::<Arc<Mutex<HashMap<Uuid, Place>>>>()?.lock().unwrap();

let place = places
    .get(&id)
    .expect("Unable to retrieve Place");

Ok(place.clone())
}

// Change back to get_or_create_place_by_name_and_country_id
pub fn get_or_create_place_by_name_and_country_id(context: &Context<'_>, name: String, country_id: Uuid) -> FieldResult<Place> {

    let mut places = context.data::<Arc<Mutex<HashMap<Uuid, Place>>>>()?.lock().unwrap();

    let res = places.iter()
        .find_map(
            |(_key, val)| 
            if val.name == name && val.country_id == country_id { 
                Some(val.clone()) 
            } else { None });

    let place = match res {
        Some(p) => p,
        None => {
            let conn = get_connection_from_context(context);

            let p = models::NewPlace::new(name, country_id);
            let place = models::Place::create(
                &conn, 
                &p)?;
            
            places.insert(place.id, place.clone());
            drop(places);
            place
        }
    };

    Ok(place.clone())
}

pub fn get_country_by_id(context: &Context<'_>, id: Uuid) -> FieldResult<Country> {

let countries = context.data::<Arc<Mutex<HashMap<Uuid, Country>>>>()?.lock().unwrap();

let country = countries
    .get(&id)
    .expect("Unable to retrieve Country");

    Ok(country.clone())
}

pub fn get_or_create_country_by_name(context: &Context<'_>, country_name: String) -> FieldResult<Country> {

let mut countries = context.data::<Arc<Mutex<HashMap<Uuid, Country>>>>()?.lock().unwrap();

let res = countries.iter()
    .find_map(|(_key, val)| if val.country_name == country_name { Some(val) } else { None });

let country = match res {
    Some(c) => c.clone(),

    // None should *rarely* happen
    None => {
        let c = NewCountry::new(country_name, 0.03);

        let conn = get_connection_from_context(context);

        // Insert country into DB
        let country = Country::create(
            &conn, 
            &c)?;
        
        // Insert into Hashmap cache
        countries.insert(country.id, country.clone());
        drop(countries);
        
        country
    }
};

    Ok(country.clone())
}

pub fn get_vaccine_by_id(context: &Context<'_>, id: Uuid) -> FieldResult<Vaccine> {
let vaccine = context.data::<HashMap<Uuid, Vaccine>>()?
    .get(&id)
    .expect("Unable to retrieve Vaccine");

    Ok(vaccine.clone())
}

pub fn get_vaccine_by_name(context: &Context<'_>, name: String) -> FieldResult<Vaccine> {
let res = context.data::<HashMap<Uuid, Vaccine>>()?
    .iter()
    .find_map(|(_key, val)| if val.vaccine_name == name { Some(val) } else { None })
    .expect("Unable to find vaccine");

    Ok(res.clone())
}


pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

// Other utilities
pub fn login(
    user_email: &str,
    user_password: &str,
    context: &Context<'_>,
) -> FieldResult<SlimUser> {
    use crate::schema::users::dsl::{email, users};

    let conn = get_connection_from_context(context);
    let user = users
        .filter(email.eq(user_email))
        .first::<User>(&conn)?;

    match verify_password(&user.hash, &user_password) {
        Ok(true) => Ok(user.into()),
        _ => Err(Error::new("No match for user / password")),
    }
}
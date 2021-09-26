use chrono::prelude::*;
use juniper::FieldResult;
use reqwest::header::VacantEntry;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use diesel::{self, Insertable, PgConnection, Queryable,
    ExpressionMethods, QueryDsl, RunQueryDsl};
use diesel_derive_enum::DbEnum;
use uuid::Uuid;
use std::collections::HashMap;

use crate::models::{Place, Country, Vaccine};
use crate::GraphQLContext;
use crate::graphql::graphql_translate;
use crate::schema::*;


#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd, Queryable, Identifiable)]
// Will assess Vaccine History against health rules engine
pub struct Vaccination {
    pub id: Uuid,
    pub vaccine_id: Uuid,
    pub dose_provider: String,
    pub location_provided_id: Uuid, // Place
    pub country_provided_id: Uuid, // Country
    pub date_time: NaiveDateTime,
    pub public_health_profile_id: Uuid,
}

// Graphql
#[graphql_object(Context = GraphQLContext)]
impl Vaccination {
    pub fn id(&self) -> FieldResult<Uuid> {
        Ok(self.id.clone())
    }

    pub fn vaccine(&self, context: &GraphQLContext) -> FieldResult<Vaccine> {
        let vaccine = context.vaccines
            .get(&self.vaccine_id)
            .expect("Unable to retrieve Vaccine");

        Ok(vaccine.clone())
    }

    pub fn location_provided(&self, context: &GraphQLContext) -> FieldResult<Place> {
        let place = context.places
            .get(&self.location_provided_id)
            .expect("Unable to retrieve Place");

        Ok(place.clone())
    }

    pub fn country_provided(&self, context: &GraphQLContext) -> FieldResult<Country> {
        let country = context.countries
            .get(&self.country_provided_id)
            .expect("Unable to retrieve Country");

        Ok(country.clone())
    }
}

impl Vaccination {
    pub fn create(conn: &PgConnection, vaccination: &NewVaccination) -> FieldResult<Vaccination> {
        let res = diesel::insert_into(vaccinations::table)
            .values(vaccination)
            .get_result(conn);

        graphql_translate(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd, Insertable)]
/// Referenced through Vaccination, QuarantinePlan, TestingHistory
#[table_name = "vaccinations"]
pub struct NewVaccination {
    pub vaccine_id: Uuid,
    pub dose_provider: String,
    pub location_provided_id: Uuid, // Place
    pub country_provided_id: Uuid, // Country
    pub date_time: NaiveDateTime,
    pub public_health_profile_id: Uuid,
}

impl NewVaccination {
    pub fn new(
        vaccine_id: Uuid,
        dose_provider: String,
        location_provided_id: Uuid, // Place
        country_provided_id: Uuid, // Country
        date_time: NaiveDateTime,
        public_health_profile_id: Uuid,
    ) -> Self {
        NewVaccination {
            vaccine_id,
            dose_provider,
            location_provided_id,
            country_provided_id,
            date_time,
            public_health_profile_id,
        }
    }
}
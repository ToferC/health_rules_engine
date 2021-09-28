use chrono::prelude::*;
use juniper::FieldResult;
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, PgConnection, Queryable,
    RunQueryDsl};
use uuid::Uuid;

use crate::models::{Place, Vaccine};
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
    pub provided_on: NaiveDateTime,
    pub public_health_profile_id: Uuid,
}

// Graphql
#[graphql_object(Context = GraphQLContext)]
impl Vaccination {
    pub fn id(&self) -> FieldResult<Uuid> {
        Ok(self.id.clone())
    }

    pub fn vaccine(&self, context: &GraphQLContext) -> FieldResult<Vaccine> {
        context.get_vaccine_by_id(self.vaccine_id)
    }

    pub fn location_provided(&self, context: &GraphQLContext) -> FieldResult<Place> {
        context.get_place_by_id(self.location_provided_id)
    }

    pub fn provided_on(&self) -> FieldResult<String> {
        Ok(self.provided_on.format("%Y-%m-%d %H:%M:%S").to_string())
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
    pub provided_on: NaiveDateTime,
    pub public_health_profile_id: Uuid,
}

impl NewVaccination {
    pub fn new(
        vaccine_id: Uuid,
        dose_provider: String,
        location_provided_id: Uuid, // Place
        provided_on: NaiveDateTime,
        public_health_profile_id: Uuid,
    ) -> Self {
        NewVaccination {
            vaccine_id,
            dose_provider,
            location_provided_id,
            provided_on,
            public_health_profile_id,
        }
    }
}
use chrono::prelude::*;
use juniper::FieldResult;
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, PgConnection, Queryable,
    RunQueryDsl, QueryDsl, ExpressionMethods};
use uuid::Uuid;

use crate::DATE_FORMAT;
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
        Ok(self.provided_on.format(DATE_FORMAT).to_string())
    }
}

impl Vaccination {
    pub fn create(conn: &PgConnection, vaccination: &NewVaccination) -> FieldResult<Vaccination> {
        let res = diesel::insert_into(vaccinations::table)
            .values(vaccination)
            .get_result(conn);

        graphql_translate(res)
    }

    pub fn get_or_create(conn: &PgConnection, vaccination: &NewVaccination) -> FieldResult<Vaccination> {
        let res = vaccinations::table
            .filter(vaccinations::public_health_profile_id.eq(&vaccination.public_health_profile_id))
            .filter(vaccinations::provided_on.eq(&vaccination.provided_on))
            .filter(vaccinations::dose_provider.eq(&vaccination.dose_provider))
            .distinct()
            .first(conn);

        let vaccination = match res {
            Ok(v) => v,
            Err(e) => {
                // Vaccination not found
                println!("{:?}", e);
                let v = Vaccination::create(conn, vaccination).expect("Unable to create person");
                v
            }
        };
        Ok(vaccination)
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
        context: &GraphQLContext,
        vaccine_name: String,
        dose_provider: String,
        location_provided_id: Uuid, // Place
        provided_on: NaiveDateTime,
        public_health_profile_id: Uuid,
    ) -> FieldResult<Self> {

        let vaccine = context.get_vaccine_by_name(vaccine_name)
            .expect("Unable to find vaccine by name");

        Ok(NewVaccination {
            vaccine_id: vaccine.id,
            dose_provider,
            location_provided_id,
            provided_on,
            public_health_profile_id,
        })
    }

    pub fn fake(
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

    pub fn from(
        context: &GraphQLContext,
        slim_vaccination: &SlimVaccination, 
        public_health_profile_id: Uuid
    ) -> FieldResult<Self> {
        let provided_country = context
            .get_or_create_country_by_name(slim_vaccination.country_provided.to_owned())
            .expect("Unable to find or create country");

        let location_provided = context.get_or_create_place_by_name_and_country_id(
            slim_vaccination.location_provided.to_owned(), provided_country.id)
            .expect("Unable to get or create origin country");

            let vaccine = context.get_vaccine_by_name(slim_vaccination.vaccine_name.to_owned())
            .expect("Unable to find vaccine by name");

        Ok(
            NewVaccination {
                vaccine_id: vaccine.id,
                dose_provider: slim_vaccination.dose_provider.to_owned(),
                location_provided_id: location_provided.id,
                provided_on: slim_vaccination.provided_on,
                public_health_profile_id,
            }
        )

    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd, GraphQLInputObject)]
/// Basic text data used to create a Vaccination object
pub struct SlimVaccination {
    pub vaccine_name: String,
    pub dose_provider: String,
    pub location_provided: String,
    pub country_provided: String, // Place
    pub provided_on: NaiveDateTime,
}
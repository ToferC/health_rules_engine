use diesel::PgConnection;
use diesel::prelude::*;
use diesel::{self, Insertable, Queryable};
use diesel::{RunQueryDsl, QueryDsl};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::prelude::*;
use chrono::Utc;
use juniper::{FieldResult};

use crate::{GraphQLContext};
use crate::graphql::{graphql_translate};
use crate::schema::*;

use crate::models::{Country, NewCountry, NewPerson, NewPlace, 
    NewPublicHealthProfile, NewTrip, NewVaccination, 
    NewVaccine, Person, Place, PublicHealthProfile, TravelGroup, 
    Trips, Vaccine, Vaccination, CovidTest};

use super::{NewCovidTest, NewQuarantinePlan};

#[derive(Debug, Clone, Serialize, Deserialize, GraphQLObject, Queryable)]
/// A struct representing the API response for a specific traveller
/// Likely to be part of a Vec<TravelResponse>
pub struct TravelResponse {
    pub id: Uuid,
    pub trip_id: Uuid,
    pub response_code: String,
    pub random_testing_referral: bool,
    pub date_time: NaiveDateTime,
    pub details: Option<String>,
}

impl TravelResponse {
    pub fn create(conn: &PgConnection, travel_response: &NewTravelResponse) -> FieldResult<TravelResponse> {
        let res = diesel::insert_into(travel_responses::table)
            .values(travel_response)
            .get_result(conn);

        graphql_translate(res)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "travel_responses"]
/// A struct representing the API response for a specific traveller
/// Likely to be part of a Vec<TravelResponse>
/// Will also be added to database for audit and data purposes.
pub struct NewTravelResponse {
    pub trip_id: Uuid,
    pub response_code: String,
    pub random_testing_referral: bool,
    pub date_time: NaiveDateTime,
    pub details: Option<String>,
}

impl NewTravelResponse {
    pub fn new(
            trip_id: Uuid,
            response_code: String,
            random_testing_referral: bool,
            details: String,
    ) -> Self {

        let details = if details != "".to_string() {
            Some(details)
        } else {
            None
        };

        NewTravelResponse {
            trip_id,
            response_code,
            random_testing_referral,
            date_time: Utc::now().naive_utc(),
            details,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Struct for data submitted by CBSA on query of ArriveCan.
/// Likely to be part of a Vec<TravelData> that will be 
/// combined into a TravelGroup.
pub struct TravelData {

    // Person (traveller) data, providing id if traveller is already
    // in the system, or a NewPerson struct if not
    pub person_id: Option<Uuid>,
    pub new_person: Option<NewPerson>,

    // Trip data
    pub trip_provider: String,
    // None for travel_identifier == private travel
    pub travel_identifier: Option<String>,
    pub booking_id: Option<String>,
    pub travel_mode: String,
    pub country_name: String,
    pub origin: String,
    pub transit_point: Vec<String>,
    pub destination: String,
    pub travel_intent: String,
    pub scheduled_departure_time: Option<NaiveDateTime>,
    pub scheduled_arrival_time: Option<NaiveDateTime>,
    pub departure_time: Option<NaiveDateTime>,
    pub arrival_time: Option<NaiveDateTime>,
    pub trip_state: String,

    // PublicHealthProfile data
    // May or may not have this detail. Will creat if not.
    pub public_health_profile_id: Option<Uuid>,
    pub smart_healthcard_pk: Option<String>,

    // Vaccinations
    // May already be in system. Likely need to do a validation
    // By date_time and provider constraint
    // Otherwise, add new vaccinations to system
    pub vaccinations: Vec<NewVaccination>,

    // CovidTest
    // Very likely to be new each time we interact, but not
    // necessarily for frequent travellers or workers
    pub covid_test: NewCovidTest,

    // QuarantinePlan
    // Also likely to be unique for each traveller.
    // Possible to be required or not required based on 
    // environment.
    pub quarantine_plan: NewQuarantinePlan,

    // Time of api post
    pub date_time: NaiveDateTime,
}

#[graphql_object(Context = GraphQLContext)]
impl TravelData {
    pub fn add_traveller(&self, context: &GraphQLContext) -> FieldResult<TravelResponse> {

        // Connect to PostgresPool
        let conn = context.pool.get().expect("Unable to connec to db");

        let mut countries = context.countries.lock().unwrap();

        // Identify country        
        let country = context.get_or_create_country_by_name(self.country_name.to_owned())?;

        // Identify or create person
         

        // Build TravelResponse
        let new_tr = NewTravelResponse::new(
            Uuid::new_v4(),
            "I".to_string(),
            true,
            "None".to_string()
        );

        let travel_response = TravelResponse::create(&conn, &new_tr)?;
        Ok(travel_response)
    }
}
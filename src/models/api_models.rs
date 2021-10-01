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
    NewPublicHealthProfile, NewTrip, NewVaccination, Trip,
    NewVaccine, Person, Place, PublicHealthProfile, TravelGroup, 
    Vaccine, Vaccination, CovidTest, SlimVaccination};

use super::trip;
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
    family_name: String,
    given_name: String,
    additional_names: Option<Vec<String>>,
    birth_date: NaiveDateTime,
    gender: String,
    travel_document_id: String,
    travel_document_issuer: String, // Country
    approved_access_level: String, // AccessLevel
    approved_access_granularity: String,

    // Trip data
    pub trip_provider: String,
    // None for travel_identifier == private travel
    pub travel_identifier: Option<String>,
    pub booking_id: Option<String>,
    pub travel_mode: String,

    // pub country_name: String,

    pub origin_name: String,
    pub origin_country_name: String,
    pub transit_points: Vec<(String, String)>, // place_name, country_name
    pub destination_name: String,
    pub destination_country_name: String,

    pub travel_intent: String,
    pub scheduled_departure_time: Option<NaiveDateTime>,
    pub scheduled_arrival_time: Option<NaiveDateTime>,
    pub departure_time: Option<NaiveDateTime>,
    pub arrival_time: Option<NaiveDateTime>,
    pub trip_state: String,

    // PublicHealthProfile data
    // May or may not have this detail. Will create if not.
    pub smart_healthcard_pk: Option<String>,

    // Vaccinations
    // May already be in system. Likely need to do a validation
    // By date_time and provider constraint
    // Otherwise, add new vaccinations to system
    pub vaccinations: Vec<SlimVaccination>,

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

    // CBSA Officer ID
    pub cbsa_officer_id: String,
}

#[graphql_object(Context = GraphQLContext)]
impl TravelData {
    pub fn parse(&self, context: &GraphQLContext) -> FieldResult<TravelResponse> {

        // Connect to PostgresPool
        let conn = context.pool.get().expect("Unable to connec to db");

        // set travel_group_id
        let travel_group_id = Uuid::new_v4();

        // Identify country        
        let country = context.get_or_create_country_by_name(self.travel_document_issuer.to_owned())?;

        // Identify or create person
        let new_person = NewPerson::new(
            self.family_name.to_owned(),
            self.given_name.to_owned(),
            self.additional_names.to_owned(),
            self.birth_date,
            self.gender.to_owned(),
            self.travel_document_id.to_owned(),
            country.id, // Country
            travel_group_id,
            self.approved_access_level.to_owned(), // AccessLevel
            self.approved_access_granularity.to_owned(),
        );

        let person = Person::get_or_create(&conn, &new_person)?;

        // Add Trip Information
        let new_trip = NewTrip::new(
            context,
            self.trip_provider.to_owned(),
            self.travel_identifier.to_owned(),
            self.booking_id.to_owned(),
            self.travel_mode.to_owned(),
            self.origin_name.to_owned(),
            self.origin_country_name.to_owned(),
            self.destination_name.to_owned(),
            self.destination_country_name.to_owned(),
            self.travel_intent.to_owned(),
            self.scheduled_departure_time,
            self.scheduled_arrival_time,
            self.departure_time,
            self.arrival_time,
            self.trip_state.to_owned(),
            travel_group_id,
            person.id,
        );

        let trip = Trip::create(&conn, &new_trip).expect("Unable to create trip");

        // Add or get PublicHealthProfile
        let profile = NewPublicHealthProfile::new(
            person.id,
            self.smart_healthcard_pk.clone(),
        );

        let public_health_profile = PublicHealthProfile::get_or_create(&conn, &profile)
            .expect("Unable to find or create profile");

        // Add vaccinations
        let mut vaccination_history: Vec<Vaccination> = Vec::new();
        
        for slim_v in &self.vaccinations {

            let nv = NewVaccination::from(
                context, 
                &slim_v, 
                public_health_profile.id).expect("Unable to create NewVaccination");

            let v = Vaccination::get_or_create(&conn, &nv)
                .expect("Unable to find or create vaccination");
            vaccination_history.push(v);
        }
        
        // Add CovidTest


        // Add QuarantinePlan



        // Call health_rules_engine
         

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
use chrono::{Duration, prelude::*};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::{self, Insertable, PgConnection, Queryable};
use diesel::{RunQueryDsl, QueryDsl};
use uuid::Uuid;
use diesel_derive_enum::DbEnum;
use juniper::{FieldResult};

use crate::schema::*;
use crate::graphql::graphql_translate;
use super::health_profile::PostalAddress;
use super::{Place, Person};

/// Travel information for a TravelGroup
/// CBSA responsible, but important for public health surveillance
#[derive(Debug, Clone, Deserialize, Serialize, GraphQLObject, Queryable, Insertable)]
#[table_name = "trips"]
pub struct Trips {
    pub id: Uuid,
    pub trip_provider: String,
    // None for travel_identifier == private travel
    pub travel_identifier: Option<String>,
    pub booking_id: Option<String>,
    pub travel_mode: String,
    pub origin_place_id: Uuid,
    pub transit_point_place_ids: Vec<Uuid>,
    pub destination_place_id: Uuid,
    pub travel_intent: String,
    pub scheduled_departure_time: Option<NaiveDateTime>,
    pub scheduled_arrival_time: Option<NaiveDateTime>,
    pub departure_time: Option<NaiveDateTime>,
    pub arrival_time: Option<NaiveDateTime>,
    pub trip_state: String,
    pub travel_group_id: Uuid,
    pub person_id: Uuid,
}

impl Trips {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn all_trips(conn: &PgConnection) -> FieldResult<Vec<Trips>> {
        let res = trips::table.load::<Trips>(conn);

        graphql_translate(res)
    }

    pub fn trip_by_id(conn: &PgConnection, id: &Uuid) -> FieldResult<Trips> {
        let res = trips::table.filter(trips::id.eq(id))
            .first(conn);

        graphql_translate(res)
    }

    pub fn person(&self, conn: &PgConnection) -> FieldResult<Person> {

        let res = persons::table.
            filter(persons::id.eq(self.person_id))
            .first(conn);

        graphql_translate(res)
    }
}

// Non Graphql
impl Trips {
    pub fn create_trip(conn: &PgConnection, trip: &NewTrip) -> FieldResult<Trips> {
        let res = diesel::insert_into(trips::table)
            .values(trip)
            .get_result(conn);

        graphql_translate(res)
    }
}

#[derive(Insertable, Debug, GraphQLInputObject)]
#[table_name = "trips"]
pub struct NewTrip {
    pub trip_provider: String,
    // None for travel_identifier == private travel
    pub travel_identifier: Option<String>,
    pub booking_id: Option<String>,
    pub travel_mode: String,
    pub origin_place_id: Uuid,
    pub transit_point_place_ids: Vec<Uuid>,
    pub destination_place_id: Uuid,
    pub travel_intent: String,
    pub scheduled_departure_time: Option<NaiveDateTime>,
    pub scheduled_arrival_time: Option<NaiveDateTime>,
    pub departure_time: Option<NaiveDateTime>,
    pub arrival_time: Option<NaiveDateTime>,
    pub trip_state: String,
    pub travel_group_id: Uuid,
    pub person_id: Uuid,
}

impl<'a> NewTrip {
    pub fn default() -> Self {

        let depart: NaiveDateTime = Utc::now().naive_utc() - Duration::days(1);
        let arrive: NaiveDateTime = Utc::now().naive_utc() + Duration::days(1);

        NewTrip { 
            trip_provider: "Air Canada".to_string(), 
            travel_identifier: Some("ADX-Q6)Y".to_string()), 
            booking_id: Some("678326432632".to_string()), 
            travel_mode: "AIR".to_string(), 
            origin_place_id: Uuid::new_v4(),
            transit_point_place_ids: vec![Uuid::new_v4()],
            destination_place_id: Uuid::new_v4(), 
            
            travel_intent: "Entry".to_string(), 
            scheduled_departure_time: Some(depart), 
            scheduled_arrival_time: Some(arrive + Duration::hours(4)), 
            departure_time: Some(depart), 
            arrival_time: Some(arrive), 
            trip_state: "planned".to_string(),
            travel_group_id: Uuid::new_v4(),
            person_id: Uuid::new_v4(),
        }
    }

    pub fn new(
        travel_group_id: &Uuid,
        person_id: &Uuid,
        origin_place_id: &Uuid,
        destination_place_id: &Uuid
    ) -> Self 
    {
        let depart: NaiveDateTime = Utc::now().naive_utc() - Duration::days(1);
        let arrive: NaiveDateTime = Utc::now().naive_utc() + Duration::days(1);

        NewTrip { 
            trip_provider: "Air Canada".to_string(), 
            travel_identifier: Some("ADX-Q6)Y".to_string()), 
            booking_id: Some("678326432632".to_string()), 
            travel_mode: "AIR".to_string(), 
            origin_place_id: origin_place_id.to_owned(), 
            transit_point_place_ids: Vec::new(), 
            
            destination_place_id: destination_place_id.to_owned(), 
            
            travel_intent: "Entry".to_string(), 
            scheduled_departure_time: Some(depart), 
            scheduled_arrival_time: Some(arrive + Duration::hours(4)), 
            departure_time: Some(depart), 
            arrival_time: Some(arrive), 
            trip_state: "planned".to_string(),
            travel_group_id: travel_group_id.to_owned(),
            person_id: person_id.to_owned(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, DbEnum)]
pub enum TripState {
    Planned,
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum TravelIntent {
    Entry,
    Exit,
    Transit,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// Should get this info from an API
/// Could be a struct with company info, contact info, API key, etc.
pub struct TravelProvider {
    id: Uuid,
    name: String,
    description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum TravelMode {
    // Strings annotate the types of travel
    // I.e., "Rail", "Bus", "Private Vehicle", "Charter"
    Air(String),
    Sea(String),
    Land(String),
}
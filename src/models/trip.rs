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

#[derive(Debug, Clone, Deserialize, Serialize, GraphQLObject, Queryable)]
/// Travel information for a TravelGroup
/// CBSA responsible, but important for public health surveillance
pub struct Trips {
    pub id: Uuid,
    pub trip_provider: String,
    // None for travel_identifier == private travel
    pub travel_identifier: Option<String>,
    pub booking_id: Option<String>,
    pub travel_mode: String,
    pub origin: String,
    pub transit_points: Vec<String>,
    pub destination: String,
    pub travel_intent: String,
    pub scheduled_departure_time: Option<NaiveDateTime>,
    pub scheduled_arrival_time: Option<NaiveDateTime>,
    pub departure_time: Option<NaiveDateTime>,
    pub arrival_time: Option<NaiveDateTime>,
    pub trip_state: String,
    pub travel_group_id: Uuid,
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
    pub origin: String,
    pub transit_points: Vec<String>,
    pub destination: String,
    pub travel_intent: String,
    pub scheduled_departure_time: Option<NaiveDateTime>,
    pub scheduled_arrival_time: Option<NaiveDateTime>,
    pub departure_time: Option<NaiveDateTime>,
    pub arrival_time: Option<NaiveDateTime>,
    pub trip_state: String,
    pub travel_group_id: Uuid,
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
            origin: "London".to_string(), 
            transit_points: vec!["Montreal".to_string()], 
            
            destination: "Winnipeg".to_string(), 
            
            travel_intent: "Entry".to_string(), 
            scheduled_departure_time: Some(depart), 
            scheduled_arrival_time: Some(arrive + Duration::hours(4)), 
            departure_time: Some(depart), 
            arrival_time: Some(arrive), 
            trip_state: "planned".to_string(),
            travel_group_id: Uuid::new_v4(),
        }
    }

    pub fn new(travel_group_id: &Uuid) -> Self {

        let depart: NaiveDateTime = Utc::now().naive_utc() - Duration::days(1);
        let arrive: NaiveDateTime = Utc::now().naive_utc() + Duration::days(1);

        NewTrip { 
            trip_provider: "Air Canada".to_string(), 
            travel_identifier: Some("ADX-Q6)Y".to_string()), 
            booking_id: Some("678326432632".to_string()), 
            travel_mode: "AIR".to_string(), 
            origin: "London".to_string(), 
            transit_points: vec!["Montreal".to_string()], 
            
            destination: "Winnipeg".to_string(), 
            
            travel_intent: "Entry".to_string(), 
            scheduled_departure_time: Some(depart), 
            scheduled_arrival_time: Some(arrive + Duration::hours(4)), 
            departure_time: Some(depart), 
            arrival_time: Some(arrive), 
            trip_state: "planned".to_string(),
            travel_group_id: travel_group_id.to_owned(),
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
/// Should get this from an API
pub enum Country {
    Canada,
    UnitedStates,
    France,
    Morocco,
    Spain,
    Brazil,
    CoteDIvoire,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// Should get this info from an API
/// Could be a struct with company info, contact info, API key, etc.
pub enum TravelProvider {
    Private,
    // Air
    AirCanada,
    AirFrance,
    United,
    PersonalCharter,
    // Sea
    RoyalCarribean,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// Will be cities, airports, ports of entry, destinations
pub enum Place {
    NewYorkCity,
    PearsonT1,
    PearsonT2,
    BillyBishop,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum TravelMode {
    // Strings annotate the types of travel
    // I.e., "Rail", "Bus", "Private Vehicle", "Charter"
    Air(String),
    Sea(String),
    Land(String),
}
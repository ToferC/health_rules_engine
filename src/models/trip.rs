use chrono::{Duration, prelude::*};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::{self, Queryable, Insertable};
use uuid::Uuid;
use diesel_derive_enum::DbEnum;
use crate::schema::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
/// People travelling together
/// Referenced through Person and links to voyage
pub struct TravelGroup {
    pub uid: String,
    pub trip_uid: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// Travel information for a TravelGroup
/// CBSA responsible, but important for public health surveillance
pub struct Trip {
    pub uid: Uuid,
    pub provider: String,
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
    pub trip_state: TripState,
}

#[derive(Insertable, Debug)]
#[table_name = "trips"]
pub struct NewTrip<'a> {
    pub uid: Uuid,
    pub provider: &'a str,
    // None for travel_identifier == private travel
    pub travel_identifier: Option<&'a str>,
    pub booking_id: Option<&'a str>,
    pub travel_mode: &'a str,
    pub origin: &'a str,
    pub transit_points: Vec<&'a str>,
    pub destination: &'a str,
    pub travel_intent: &'a str,
    pub scheduled_departure_time: Option<NaiveDateTime>,
    pub scheduled_arrival_time: Option<NaiveDateTime>,
    pub departure_time: Option<NaiveDateTime>,
    pub arrival_time: Option<NaiveDateTime>,
    pub trip_state: TripState,
}

impl<'a> NewTrip<'a> {
    pub fn default() -> Self {

        let depart: NaiveDateTime = Utc::now().naive_utc() - Duration::days(1);
        let arrive: NaiveDateTime = Utc::now().naive_utc() + Duration::days(1);

        NewTrip { 
            uid: Uuid::new_v4(),
            provider: "Air Canada", 
            travel_identifier: Some("ADX-Q6)Y"), 
            booking_id: Some("678326432632"), 
            travel_mode: "AIR", 
            origin: "London", 
            transit_points: vec!["Montreal"], 
            
            destination: "Winnipeg", 
            
            travel_intent: "Entry", 
            scheduled_departure_time: Some(depart), 
            scheduled_arrival_time: Some(arrive + Duration::hours(4)), 
            departure_time: Some(depart), 
            arrival_time: Some(arrive), 
            trip_state: TripState::Planned,
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
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
// use diesel::prelude::*;

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
    pub uid: String,
    pub provider: TravelProvider,
    // None for travel_identifier == private travel
    pub travel_identifier: Option<String>,
    pub booking_id: Option<String>,
    pub travel_mode: TravelMode,
    pub origin: Place,
    pub transit_points: Vec<Place>,
    pub destination: Place,
    pub travel_intent: TravelIntent,
    pub scheduled_departure_time: Option<NaiveDateTime>,
    pub scheduled_arrival_time: Option<NaiveDateTime>,
    pub departure_time: Option<NaiveDateTime>,
    pub arrival_time: Option<NaiveDateTime>,
    pub trip_state: TripState,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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
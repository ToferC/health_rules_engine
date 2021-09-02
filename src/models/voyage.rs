use chrono::prelude::*;
use serde::{Deserialize, Serialize};
// use diesel::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
/// People travelling together
/// Referenced through Person and links to voyage
pub struct TravelGroup {
    pub uid: String,
    pub voyage_uid: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// Travel information for a TravelGroup
/// CBSA responsible, but important for public health surveillance
pub struct Voyage {
    pub uid: String,
    pub travel_provider: TravelProvider,
    // None for travel_identifier == private travel
    pub travel_identifier: Option<String>,
    pub booking_id: Option<String>,
    pub travel_mode: TravelMode,
    pub origin: TravelHub,
    pub transit_points: Vec<TransitPoint>,
    pub destination: TravelHub,
    pub border_point: TravelHub,
    pub travel_intent: TravelIntent,
    pub scheduled_departure_datetime: Option<NaiveDateTime>,
    pub scheduled_arrival_datetime: Option<NaiveDateTime>,
    pub departure_datetime: Option<NaiveDateTime>,
    pub arrival_datetime: Option<NaiveDateTime>,
    pub voyage_state: VoyageState,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransitPoint {
    pub travel_hub: TravelHub,
    pub transit_date: NaiveDate,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum VoyageState {
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
pub enum TravelHub {
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
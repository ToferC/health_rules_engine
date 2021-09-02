use chrono::prelude::*;
use serde::{Deserialize, Serialize};
// use diesel::prelude::*;

use super::health_profile::{QuarantinePlan, TestingHistory};

#[derive(Debug, Clone, Deserialize, Serialize)]
// People travelling together
pub struct TravelGroup {
    pub uid: String,
    pub voyage_uid: String,
    pub people_uids: Vec<String>,
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
    pub transit_points: Vec<TravelHub>,
    pub destination: TravelHub,
    pub border_point: TravelHub,
    pub travel_intent: TravelIntent,
    pub scheduled_departure_datetime: Option<NaiveDateTime>,
    pub scheduled_arrival_datetime: Option<NaiveDateTime>,
    pub departure_datetime: Option<NaiveDateTime>,
    pub arrival_datetime: Option<NaiveDateTime>,
    pub voyage_state: VoyageState,

    // Where should these go??
    pub quarantine_plan: QuarantinePlan,
    pub testing_history: Vec<TestingHistory>,
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Address{}
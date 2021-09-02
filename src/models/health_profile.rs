use chrono::prelude::*;
use serde::{Deserialize, Serialize};
// use diesel::prelude::*;

use super::voyage::{Country, TravelHub};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PublicHealthProfile {
    pub uid: String,
    pub person_uuid: String,
    pub smart_healthcard_pk: String,
    // OR
    pub vaccination_history: Vec<Vaccination>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
// Will assess Vaccine History against health rules engine
pub struct Vaccination {
    uid: String,
    dose: Vaccine,
    provider: String,
    location_provided: String, // or TravelHub renamed
    country_provided: Country,
    date_time: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Vaccine {
    uid: String,
    maker: String,
    approved: bool,
    details: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QuarantinePlan {
    pub uid: String,
    pub date_created: NaiveDateTime,
    pub confirmation_quarantine: bool,
    pub confirmation_no_vulnerable: bool,
    pub address: Address,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TestingHistory{}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Address {
    pub street_number: i32,
    pub unit_number: Option<i32>,
    pub street_name: String,
    pub additional_info: Option<String>,
    pub city: TravelHub,
    pub province_state: String,
    pub country: Country,
}


use chrono::prelude::*;
use serde::{Deserialize, Serialize};
// use diesel::prelude::*;

use super::voyage::{TravelHub, Country};
use super::health_profile::PublicHealthProfile;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Person {
    pub uid: String,
    pub date_of_birth: NaiveDate,
    pub travel_document_issuer: Country,
    pub travel_document_uid: String,
    pub public_health_profile: PublicHealthProfile,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Address {
    pub street_number: i32,
    pub unit_number: Option<i32>,
    pub city: TravelHub,
    pub province_state: String,
    pub country: Country,
}

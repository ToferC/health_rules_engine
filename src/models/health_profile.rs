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
    pub quarantine_plan: QuarantinePlan,
    pub testing_history: Vec<TestingHistory>,
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
    pub quarantine_required: bool,
    pub confirmation_no_vulnerable: bool,
    pub address: Address,
    pub compliance_check: bool,
    pub compliance_check_result: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CheckInResult {
    pub uid: String,
    pub quarantine_plan_uid: String,
    pub date_time: NaiveDateTime,
    pub check_in_complete: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TestingHistory{
    pub uid: String,
    pub public_health_profile_uid: String,
    pub test: String,
    pub test_type: TestType,
    pub date_taken: NaiveDateTime,
    pub test_result: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AccessLevel {
    Adminstrator,
    Analyst,
    Employee,
    Research,
    Open,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum TestType {
    Molecular,
    Other,
}

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


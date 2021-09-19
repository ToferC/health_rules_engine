use chrono::prelude::*;
use serde::{Deserialize, Serialize};
// use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use uuid::Uuid;

use super::{Place, Country};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PublicHealthProfile {
    pub id: String,
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
    id: Uuid,
    dose: Vaccine,
    provider: String,
    location_provided: String, // or TravelHub renamed
    country_provided: Country,
    date_time: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Vaccine {
    id: Uuid,
    maker: String,
    approved: bool,
    details: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QuarantinePlan {
    pub id: Uuid,
    pub date_created: NaiveDateTime,
    pub quarantine_required: bool,
    pub confirmation_no_vulnerable: bool,
    pub address: PostalAddress,
    pub compliance_check: bool,
    pub compliance_check_result: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CheckInResult {
    pub id: Uuid,
    pub quarantine_plan_uid: String,
    pub date_time: NaiveDateTime,
    pub check_in_complete: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TestingHistory{
    pub id: Uuid,
    pub public_health_profile_uid: String,
    pub test: String,
    pub test_type: TestType,
    pub date_taken: NaiveDateTime,
    pub test_result: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, DbEnum)]
#[DieselType = "Access_level_enum"]
pub enum AccessLevelEnum {
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
pub struct GeoCoordinates {
    pub latitude: f32,
    pub longitude: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PostalAddress {
    pub id: Uuid,
    pub street_address: String,
    pub address_locality: Place,
    pub address_region: String,
    pub address_country: Country,
    pub postal_code: String,
    pub geo: GeoCoordinates,
    pub additional_info: Option<String>,
}


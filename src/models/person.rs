use chrono::prelude::*;
use serde::{Deserialize, Serialize};
// use diesel::prelude::*;

use super::trip::{Country};
use super::access_log::{AccessLevel, Granularity};

#[derive(Debug, Clone, Deserialize, Serialize)]
/// Linked from HealthProfile
pub struct Person {
    pub uid: String,
    pub birth_date: NaiveDate,
    pub travel_document_issuer: Country,
    pub approved_access_level: AccessLevel,
    pub approved_access_granularity: Granularity,
    
    // Relations
    pub travel_document_uid: String,
    pub travel_group_uid: String,
}

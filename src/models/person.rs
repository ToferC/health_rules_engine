use chrono::prelude::*;
use serde::{Deserialize, Serialize};
// use diesel::prelude::*;

use super::voyage::{Country};

#[derive(Debug, Clone, Deserialize, Serialize)]
/// Linked from HealthProfile
pub struct Person {
    pub uid: String,
    pub date_of_birth: NaiveDate,
    pub travel_document_issuer: Country,
    pub travel_document_uid: String,
    pub travel_group_uid: String,
}

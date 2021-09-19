use chrono::prelude::*;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
// use diesel::prelude::*;

// use super::trip::{Country};
// use super::access_log::{AccessLevel, Granularity};
use crate::schema::*;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable)]
/// Linked from HealthProfile
/// Linked to Trip
pub struct Person {
    pub id: Uuid,
    pub birth_date: NaiveDate,
    pub travel_document_issuer: String, // Country
    pub approved_access_level: String, // AccessLevel
    pub approved_access_granularity: String, // Granularity
    pub trip_id: Uuid,
    
    // Relations
    pub travel_document_id: Uuid,
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
/// Linked from HealthProfile
/// Linked to Trip
#[table_name = "persons"]
pub struct NewPerson {
    pub birth_date: NaiveDateTime,
    pub travel_document_issuer: Uuid, // Country
    pub approved_access_level: String, // AccessLevel
    pub approved_access_granularity: String, // Granularity
    pub trip_id: Uuid,
    
    // Relations
    pub travel_document_id: Uuid,
}

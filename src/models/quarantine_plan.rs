use chrono::prelude::*;
use juniper::FieldResult;
use reqwest::header::VacantEntry;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use diesel::{self, Insertable, PgConnection, Queryable,
    ExpressionMethods, QueryDsl, RunQueryDsl};
use diesel_derive_enum::DbEnum;
use uuid::Uuid;
use std::collections::HashMap;

use crate::models::{Place, Country};
use crate::GraphQLContext;
use crate::graphql::graphql_translate;
use crate::schema::*;

#[derive(Debug, Clone, Deserialize, Serialize, GraphQLObject, Insertable, Queryable)]
#[table_name = "quarantine_plans"]
pub struct QuarantinePlan {
    pub id: Uuid,
    pub public_health_profile_id: Uuid,
    pub date_created: NaiveDateTime,
    pub quarantine_required: bool,
    pub confirmation_no_vulnerable: bool,
    pub postal_address_id: Uuid, // PostalAddress
    pub compliance_check: bool,
    pub compliance_check_result: bool,
    pub active: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, GraphQLObject, Insertable)]
#[table_name = "quarantine_plans"]
pub struct NewQuarantinePlan {
    pub public_health_profile_id: Uuid,
    pub date_created: NaiveDateTime,
    pub quarantine_required: bool,
    pub confirmation_no_vulnerable: bool,
    pub postal_address_id: Uuid, // PostalAddress
    pub compliance_check: bool,
    pub compliance_check_result: bool,
    pub active: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, GraphQLObject)]
pub struct CheckInResult {
    pub id: Uuid,
    pub quarantine_plan_id: Uuid,
    pub user_id: Uuid,
    pub date_time: NaiveDateTime,
    pub check_in_complete: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, GraphQLObject)]
pub struct NewCheckInResult {
    pub quarantine_plan_id: Uuid,
    pub user_id: Uuid,
    pub date_time: NaiveDateTime,
    pub check_in_complete: bool,
}
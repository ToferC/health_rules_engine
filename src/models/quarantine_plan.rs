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

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, Queryable)]
#[table_name = "quarantine_plans"]
/// Referenced by compliance check
/// References public_health_profile
pub struct QuarantinePlan {
    pub id: Uuid,
    pub public_health_profile_id: Uuid,
    pub date_created: NaiveDateTime,
    pub quarantine_required: bool,
    pub confirmation_no_vulnerable: bool,
    pub postal_address_id: String, // PostalAddress
    pub active: bool,
}

#[graphql_object(Context = GraphQLContext)]
impl QuarantinePlan {
    pub fn id(&self) -> FieldResult<Uuid> {
        Ok(self.id.clone())
    }

    pub fn date_created(&self) -> FieldResult<String> {
        Ok(self.date_created.format("%Y-%m-%d").to_string())
    }

    pub fn quarantine_required(&self) -> FieldResult<bool> {
        Ok(self.quarantine_required)
    }

    pub fn confirmation_no_vulnerable(&self) -> FieldResult<bool> {
        Ok(self.confirmation_no_vulnerable)
    }

    pub fn postal_address_id(&self) -> FieldResult<String> {
        Ok(self.postal_address_id.to_owned())
    }

    pub fn active(&self) -> FieldResult<bool> {
        Ok(self.active)
    }

    pub fn check_in_history(&self, context: &GraphQLContext) -> FieldResult<Vec<CheckInResult>> {
        let conn = context.pool.get().expect("Unable to connect to DB");

        let res = check_in_results::table
            .filter(check_in_results::quarantine_plan_id.eq(self.id))
            .load::<CheckInResult>(&conn);

        graphql_translate(res)
    }
}

impl QuarantinePlan {
    pub fn create(conn: &PgConnection, plan: &NewQuarantinePlan) -> FieldResult<QuarantinePlan> {
        let res = diesel::insert_into(quarantine_plans::table)
            .values(plan)
            .get_result(conn);

        graphql_translate(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, GraphQLObject, Insertable)]
#[table_name = "quarantine_plans"]
pub struct NewQuarantinePlan {
    pub public_health_profile_id: Uuid,
    pub date_created: NaiveDateTime,
    pub quarantine_required: bool,
    pub confirmation_no_vulnerable: bool,
    pub postal_address_id: String, // PostalAddress
    pub active: bool,
}

impl NewQuarantinePlan {
    pub fn new(
        public_health_profile_id: Uuid,
        date_created: NaiveDateTime,
        quarantine_required: bool,
        confirmation_no_vulnerable: bool,
        postal_address_id: String, // PostalAddress
        active: bool,
    ) -> Self {
        NewQuarantinePlan {
            public_health_profile_id,
            date_created,
            quarantine_required,
            confirmation_no_vulnerable,
            postal_address_id,
            active,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, GraphQLObject, Queryable)]
pub struct CheckInResult {
    pub id: Uuid,
    pub quarantine_plan_id: Uuid,
    pub verifier_id: Uuid,
    pub date_time: NaiveDateTime,
    pub check_in_complete: bool,
}

impl CheckInResult {
    pub fn create(conn: &PgConnection, check_in: &NewCheckInResult) -> FieldResult<CheckInResult> { 
        let res = diesel::insert_into(check_in_results::table)
            .values(check_in)
            .get_result(conn);
        
        graphql_translate(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, GraphQLObject, Insertable)]
#[table_name = "check_in_results"]
pub struct NewCheckInResult {
    pub quarantine_plan_id: Uuid,
    pub verifier_id: Uuid,
    pub date_time: NaiveDateTime,
    pub check_in_complete: bool,
}

impl NewCheckInResult {
    pub fn new(
        quarantine_plan_id: Uuid,
        verifier_id: Uuid,
        date_time: NaiveDateTime,
        check_in_complete: bool,
    ) -> Self {
        NewCheckInResult {
            quarantine_plan_id,
            verifier_id,
            date_time,
            check_in_complete,
        }
    }
}
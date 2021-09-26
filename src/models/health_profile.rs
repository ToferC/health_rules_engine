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

use crate::models::{Place, Country, Vaccination, Vaccine,
    QuarantinePlan};
use crate::GraphQLContext;
use crate::graphql::graphql_translate;
use crate::schema::*;


#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd, Insertable, Queryable)]
/// Referenced through Vaccination, QuarantinePlan, TestingHistory
pub struct PublicHealthProfile {
    pub id: Uuid,
    pub person_id: Uuid,
    pub smart_healthcard_pk: String,
}

// GraphQL Implementation
#[graphql_object(Context = GraphQLContext)]
impl PublicHealthProfile {
    pub fn id(&self) -> FieldResult<Uuid> {
        Ok(self.id.clone())
    }

    pub fn person_id(&self) -> FieldResult<Uuid> {
        Ok(self.person_id.clone())
    }

    pub fn smart_healthcard_pk(&self) -> FieldResult<String> {
        Ok(self.smart_healthcard_pk.to_owned())
    }

    pub fn vaccination_history(&self, context: &GraphQLContext) -> FieldResult<Vec<Vaccination>> {
        let conn = context.pool.get().expect("Unable to connect to DB");

        let res = vaccinations::table
            .filter(vaccinations::public_health_profile_id.eq(self.id))
            .load::<Vaccination>(&conn);

        graphql_translate(res)
    }

    pub fn testing_history(&self, context: &GraphQLContext) -> FieldResult<Vec<TestingHistory>> {
        let conn = context.pool.get().expect("Unable to connect to DB");

        let res = testing_history::table
            .filter(testing_history::public_health_profile_id.eq(self.id))
            .load::<TestingHistory>(&conn);

        graphql_translate(res)
    }

    pub fn quarantine_plans(&self, context: &GraphQLContext) -> FieldResult<Vec<QuarantinePlan>> {
        let conn = context.pool.get().expect("Unable to connect to DB");

        let res = quarantine_plans::table
            .filter(quarantine_plans::public_health_profile_id.eq(self.id))
            .load::<QuarantinePlan>(&conn);

        graphql_translate(res)
    }
}

impl PublicHealthProfile {
    pub fn create(conn: &PgConnection, profile: &NewPublicHealthProfile) -> FieldResult<PublicHealthProfile> { 
        let res = diesel::insert_into(public_health_profiles::table)
            .values(profile)
            .get_result(conn);
        
        graphql_translate(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd, Insertable)]
/// Referenced through Vaccination, QuarantinePlan, TestingHistory
#[table_name = "public_health_profiles"]
pub struct NewPublicHealthProfile {
    pub person_id: Uuid,
    pub smart_healthcard_pk: String,
}

impl NewPublicHealthProfile {
    pub fn new(
        person_id: Uuid,
        smart_healthcard_pk: String,
    ) -> Self {
        NewPublicHealthProfile {
            person_id,
            smart_healthcard_pk,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, GraphQLObject, Insertable, Queryable)]
#[table_name = "testing_history"]
pub struct TestingHistory{
    pub id: Uuid,
    pub public_health_profile_id: Uuid,
    pub test: String,
    pub test_type: String, // TestType
    pub date_taken: NaiveDateTime,
    pub test_result: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, DbEnum, GraphQLEnum)]
#[DieselType = "Access_level_enum"]
pub enum AccessLevelEnum {
    Adminstrator,
    Analyst,
    Employee,
    Research,
    Open,
}

#[derive(Debug, Clone, Deserialize, Serialize, GraphQLEnum)]
pub enum TestType {
    Molecular,
    Other,
}

#[derive(Debug, Clone, Deserialize, Serialize, GraphQLObject)]
pub struct GeoCoordinates {
    pub latitude: f64,
    pub longitude: f64,
}
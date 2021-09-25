use chrono::prelude::*;
use juniper::FieldResult;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use diesel::{QueryDsl, RunQueryDsl, ExpressionMethods};
use diesel_derive_enum::DbEnum;
use uuid::Uuid;

use crate::models::{Place, Country};
use crate::GraphQLContext;
use crate::graphql::graphql_translate;
use crate::schema::*;


#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd, Insertable)]
/// Referenced through Vaccination, QuarantinePlan, TestingHistory
pub struct PublicHealthProfile {
    pub id: Uuid,
    pub person_id: Uuid,
    pub smart_healthcard_pk: String,
}

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

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd, Queryable, Identifiable, GraphQLObject)]
// Will assess Vaccine History against health rules engine
pub struct Vaccination {
    pub id: Uuid,
    pub vaccine_id: Uuid,
    pub dose_provider: String,
    pub location_provided_id: Uuid, // Place
    pub country_provided_id: Uuid, // Country
    pub date_time: NaiveDateTime,
    pub public_health_profile_id: Uuid,
}

#[derive(Debug, Clone, Deserialize, Serialize, GraphQLObject, Insertable, Queryable)]
#[table_name = "vaccines"]
pub struct Vaccine {
    pub id: Uuid,
    pub maker: String,
    pub approved: bool,
    pub details: String,
}

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

#[derive(Debug, Clone, Deserialize, Serialize, GraphQLObject)]
pub struct CheckInResult {
    pub id: Uuid,
    pub quarantine_plan_id: Uuid,
    pub user_id: Uuid,
    pub date_time: NaiveDateTime,
    pub check_in_complete: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, GraphQLObject, Insertable, Queryable)]
#[table_name = "testing_history"]
pub struct TestingHistory{
    pub id: Uuid,
    pub public_health_profile_id: Uuid,
    pub test: String,
    pub test_type: String,
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

#[derive(Debug, Clone, Deserialize, Serialize, GraphQLObject)]
pub struct PostalAddress {
    pub id: Uuid,
    pub street_address: String,
    pub address_locality_id: Uuid,
    pub address_region: String,
    pub address_country_id: Uuid,
    pub postal_code: String,
    pub lattitude: f64,
    pub longitude: f64,
    pub additional_info: Option<String>,
}


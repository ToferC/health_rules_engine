use std::fmt::Debug;

use chrono::{Duration, prelude::*};
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, PgConnection, Queryable, ExpressionMethods};
use diesel::{RunQueryDsl, QueryDsl};
use uuid::Uuid;
use juniper::{FieldResult};
use rand::{Rng, thread_rng};

use crate::schema::*;
use crate::graphql::graphql_translate;
use crate::GraphQLContext;
use crate::models::{Country};

// use super::trip::{Country};
// use super::access_log::{AccessLevel, Granularity};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "persons"]
/// Linked from HealthProfile
/// Linked to Trip
pub struct Person {
    pub id: Uuid,
    pub birth_date: NaiveDateTime,
    pub travel_document_issuer_id: Uuid, // Country
    pub approved_access_level: String, // AccessLevel
    pub approved_access_granularity: String, // Granularity
    
    // Relations
    pub travel_document_id: Uuid,
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, GraphQLInputObject)]
/// Linked from HealthProfile
/// Linked to Trip
#[table_name = "persons"]
pub struct NewPerson {
    pub birth_date: NaiveDateTime,
    pub travel_document_issuer_id: Uuid, // Country
    pub approved_access_level: String, // AccessLevel
    pub approved_access_granularity: String, // Granularity
    
    // Relations
    pub travel_document_id: Uuid,
}

impl NewPerson {
    pub fn new() -> NewPerson {

        let mut rng = thread_rng();
        let random_year = rng.gen_range(1945..2002);
        let random_month = rng.gen_range(1..13);
        let random_day = rng.gen_range(1..29);

        let dob: NaiveDateTime = Utc.ymd(random_year, random_month, random_day).and_hms(1, 1, 1).naive_utc();
        
        NewPerson {
            birth_date: dob,
            travel_document_issuer_id: Uuid::new_v4(),
            approved_access_level: "medical_records".to_string(),
            approved_access_granularity: "aggregated".to_string(),
            travel_document_id: Uuid::new_v4(),
        }
    }
}

// Non Graphql
impl Person {
    pub fn create(conn: &PgConnection, person: &NewPerson) -> FieldResult<Person> {
        let res = diesel::insert_into(persons::table)
            .values(person)
            .get_result(conn);

        graphql_translate(res)
    }
}

#[graphql_object(Context = GraphQLContext)]
impl Person {
    pub fn birth_date(&self) -> FieldResult<NaiveDateTime> {
        Ok(self.birth_date)
    }

    pub fn approved_access_level(&self) -> FieldResult<String> {
        Ok(self.approved_access_level.to_owned())
    }

    pub fn approved_access_granularity(&self) -> FieldResult<String> {
        Ok(self.approved_access_granularity.to_owned())
    }

    pub fn country(&self, context: &GraphQLContext) -> FieldResult<Country> {

        let conn = context.pool.get().expect("Unable to connect to DB");

        let res = countries::table
            .filter(countries::id.eq(self.travel_document_issuer_id))
            .first(&conn);

        graphql_translate(res)
    }


}


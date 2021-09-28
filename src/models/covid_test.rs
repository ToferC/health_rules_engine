use chrono::prelude::*;
use juniper::FieldResult;
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, PgConnection, Queryable,
    RunQueryDsl};
use uuid::Uuid;

use crate::GraphQLContext;
use crate::graphql::graphql_translate;
use crate::schema::*;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "covid_tests"]
pub struct CovidTest {
    pub id: Uuid,
    pub public_health_profile_id: Uuid,
    pub test_name: String,
    pub test_type: String, // TestType
    pub date_taken: NaiveDateTime,
    pub test_result: bool,
}

#[graphql_object(Context = GraphQLContext)]
impl CovidTest {
    pub fn id(&self) -> FieldResult<Uuid> {
        Ok(self.id)
    }

    pub fn test_name(&self) -> FieldResult<String> {
        Ok(self.test_name.to_owned())
    }

    pub fn test_type(&self) -> FieldResult<String> {
        Ok(self.test_type.to_owned())
    }

    pub fn date_taken(&self) -> FieldResult<String> {
        Ok(self.date_taken.format("%Y-%m-%d").to_string())
    }

    pub fn test_result(&self) -> FieldResult<bool> {
        Ok(self.test_result)
    }
}

impl CovidTest {
    pub fn create(conn: &PgConnection, test: &NewCovidTest) -> FieldResult<CovidTest> {
        let res = diesel::insert_into(covid_tests::table)
            .values(test)
            .get_result(conn);
        
        graphql_translate(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, GraphQLObject, Insertable)]
#[table_name = "covid_tests"]
pub struct NewCovidTest {
    pub public_health_profile_id: Uuid,
    pub test_name: String,
    pub test_type: String, // TestType
    pub date_taken: NaiveDateTime,
    pub test_result: bool,
}

impl NewCovidTest {
    pub fn new(
        public_health_profile_id: Uuid,
        test_name: String,
        test_type: String, // TestType
        date_taken: NaiveDateTime,
        test_result: bool,
    ) -> Self {
        NewCovidTest {
            public_health_profile_id,
            test_name,
            test_type,
            date_taken,
            test_result,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, GraphQLEnum)]
pub enum TestType {
    Molecular,
    Other,
}
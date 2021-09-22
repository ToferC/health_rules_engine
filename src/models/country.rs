use crate::PgConnection;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::{self, Insertable, Queryable};
use diesel::{RunQueryDsl, QueryDsl};
use juniper::{FieldResult, FieldError};
use uuid::Uuid;

use crate::graphql::graphql_translate;
use crate::schema::*;

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
#[table_name = "countries"]
pub struct NewCountry {
    country_name: String,
}

impl NewCountry {
    pub fn new(country_name: String) -> Self {
        NewCountry {
            country_name,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable, GraphQLObject)]
#[table_name = "countries"]
/// Should get this from an API
pub struct Country {
    pub id: Uuid,
    pub country_name: String,
}

impl Country {
    pub fn create(conn: &PgConnection, country: &NewCountry) -> FieldResult<Country> {
        let res = diesel::insert_into(countries::table)
            .values(country)
            .get_result(conn);

        graphql_translate(res)
    }

    pub fn get_by_id(conn: &PgConnection, id: &Uuid) -> FieldResult<Country> {
        let res = countries::table.filter(countries::id.eq(id))
            .first(conn);

        graphql_translate(res)
    }
}

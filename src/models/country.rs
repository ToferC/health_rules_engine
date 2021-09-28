use std::collections::HashMap;

use crate::PgConnection;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::{self, Insertable, Queryable};
use diesel::{RunQueryDsl, QueryDsl};
use juniper::{FieldResult};
use uuid::Uuid;

use crate::graphql::graphql_translate;
use crate::schema::*;

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
#[table_name = "countries"]
pub struct NewCountry {
    country_name: String,
    risk_rate: f64,
}

impl NewCountry {
    pub fn new(country_name: String, risk_rate: f64) -> Self {
        NewCountry {
            country_name,
            risk_rate,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable, GraphQLObject)]
#[table_name = "countries"]
/// Should get this from an API
pub struct Country {
    pub id: Uuid,
    pub country_name: String,
    pub risk_rate: f64,
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

    pub fn load_into_hash(conn: &PgConnection) -> HashMap<Uuid, Country> {
        let res = countries::table
            .load::<Country>(conn)
            .expect("Unable to load countries");

        let mut countries: HashMap<Uuid, Country> = HashMap::new();
        for c in res {
            countries.insert(c.id, c);
        };

        countries 
    }
}

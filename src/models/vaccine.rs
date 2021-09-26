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
#[table_name = "vaccines"]
pub struct Vaccine {
    pub id: Uuid,
    pub maker: String,
    pub approved: bool,
    pub details: String,
}

impl Vaccine {
    pub fn create(conn: &PgConnection, vaccine: &NewVaccine) -> FieldResult<Vaccine> {
        let res = diesel::insert_into(vaccines::table)
            .values(vaccine)
            .get_result(conn);

        graphql_translate(res)
    }

    pub fn load_into_hash(conn: &PgConnection) -> HashMap<Uuid, Vaccine> {
        let res = vaccines::table
            .load::<Vaccine>(conn)
            .expect("Unable to load countries");

        let mut new_map: HashMap<Uuid, Vaccine> = HashMap::new();
        for v in res {
            new_map.insert(v.id, v);
        };

        new_map 
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd, Insertable)]
/// Referenced through Vaccination, QuarantinePlan, TestingHistory
#[table_name = "vaccines"]
pub struct NewVaccine {
    pub maker: String,
    pub approved: bool,
    pub details: String,
}

impl NewVaccine {
    pub fn new(
        maker: String,
        approved: bool,
        details: String,
    ) -> Self {
        NewVaccine {
            maker,
            approved,
            details,
        }
    }
}
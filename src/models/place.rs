use crate::PgConnection;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::{self, Insertable, Queryable};
use diesel::{RunQueryDsl, QueryDsl};
use juniper::{FieldResult, FieldError};
use uuid::Uuid;

use crate::graphql::graphql_translate;
use crate::schema::*;


#[derive(Debug, Clone, Deserialize, Serialize, Queryable)]
/// Will be cities, airports, ports of entry, destinations
/// Referenced by PostalAddress
pub struct Place {
    pub id: Uuid,
    pub name: String,
}

impl Place {
    pub fn create(conn: &PgConnection, place: &NewPlace) -> FieldResult<Self> {
        let res = diesel::insert_into(places::table)
            .values(place)
            .get_result(conn);

        graphql_translate(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
/// Will be cities, airports, ports of entry, destinations
/// Referenced by PostalAddress
#[table_name = "places"]
pub struct NewPlace {
    pub place_name: String,
}

impl NewPlace {
    pub fn new(place_name: String) -> Self {
        NewPlace { place_name }
    }
}
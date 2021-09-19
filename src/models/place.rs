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
    id: Uuid,
    name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
/// Will be cities, airports, ports of entry, destinations
/// Referenced by PostalAddress
#[table_name = "places"]
pub struct NewPlace {
    place_name: String,
}

impl NewPlace {
    pub fn new(place_name: String) -> Self {
        NewPlace { place_name }
    }

    pub fn create_place(conn: &PgConnection, place: &NewPlace) -> FieldResult<Place> {
        let res = diesel::insert_into(places::table)
            .values(place)
            .get_result(conn);

        graphql_translate(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// Should get this from an API
pub struct Country {
    id: Uuid,
    place_name: String,
}
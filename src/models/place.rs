use crate::PgConnection;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::{self, Insertable, Queryable};
use diesel::{RunQueryDsl, QueryDsl};
use juniper::{FieldResult, FieldError};
use uuid::Uuid;

use crate::graphql::graphql_translate;
use crate::models::Country;
use crate::GraphQLContext;
use crate::schema::*;


#[derive(Debug, Clone, Deserialize, Serialize, Queryable)]
/// Will be cities, airports, ports of entry, destinations
/// Referenced by PostalAddress
pub struct Place {
    pub id: Uuid,
    pub name: String,
    pub country_id: Uuid,
}

impl Place {
    pub fn create(conn: &PgConnection, place: &NewPlace) -> FieldResult<Self> {
        let res = diesel::insert_into(places::table)
            .values(place)
            .get_result(conn);

        graphql_translate(res)

    }
}

#[graphql_object(Context = GraphQLContext)]
impl Place {
    pub fn id(&self) -> FieldResult<Uuid> {
        Ok(self.id)
    }

    pub fn name(&self) -> FieldResult<String> {
        Ok(self.name.to_owned())
    }

    pub fn country(&self, context: &GraphQLContext) -> FieldResult<Country> {

        let conn = context.pool.get().expect("Unable to connect to DB");

        let res = countries::table
            .filter(countries::id.eq(self.country_id))
            .first(&conn);

        graphql_translate(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable)]
/// Will be cities, airports, ports of entry, destinations
/// Referenced by PostalAddress
#[table_name = "places"]
pub struct NewPlace {
    pub place_name: String,
    pub country_id: Uuid,
}

impl NewPlace {
    pub fn new(place_name: String, country_id: Uuid) -> Self {
        NewPlace { place_name, country_id }
    }
}
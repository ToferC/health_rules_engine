use chrono::{Duration, prelude::*};
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, PgConnection, Queryable};
use diesel::{RunQueryDsl};
use uuid::Uuid;
use diesel_derive_enum::DbEnum;
use juniper::{FieldResult, graphql_object, graphql_value};

use crate::schema::*;
use crate::graphql::graphql_translate;
use super::{Trips};

#[derive(Debug, Clone, Deserialize, Serialize, GraphQLObject)]
#[serde(rename_all= "snake_case")]
/// People travelling together
/// Referenced through Person, Trip and links to voyage
pub struct TravelGroups {
    pub id: String,
}

impl TravelGroups {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn all_travel_groups(conn: &PgConnection) -> FieldResult<Vec<TravelGroups>> {
        let res = travel_groups::table.load::<TravelGroups>(conn);

        graphql_translate(res)
    }

    pub fn create_travel_group(conn: &PgConnection, travel_group: NewTravelGroup) -> FieldResult<TravelGroup> {
        let res = diesel::insert_into(travel_groups::table)
            .values(travel_group)
            .get_result(conn);

        graphql_translate(res)
    }

}

#[derive(Insertable, Debug, GraphQLInputObject)]
#[table_name = "travel_groups"]
pub struct NewTravelGroup {
    pub id: String,
}
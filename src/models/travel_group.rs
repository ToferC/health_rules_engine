use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::{self, Insertable, PgConnection, Queryable};
use diesel::{RunQueryDsl, QueryDsl};
use uuid::Uuid;
use juniper::{FieldResult};

use crate::schema::*;
use crate::graphql::{graphql_translate};
use crate::GraphQLContext;
use super::{Trips};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, PartialEq, PartialOrd, Identifiable)]
#[serde(rename_all= "snake_case")]
#[table_name = "travel_groups"]
/// People travelling together
/// Referenced through Person, Trip and links to voyage
pub struct TravelGroup {
    pub id: Uuid,
}

#[juniper::graphql_object(Context = GraphQLContext)]
impl TravelGroup {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn trips(&self, ctx: &GraphQLContext) -> Vec<Trips> {
        let conn = ctx.pool.get().expect("Unable to connect to DB");

        let res = trips::table.
            filter(trips::travel_group_id.eq(self.id))
            .load::<Trips>(&conn);

        res.unwrap()
    }
}

/// Non-Graphql
impl TravelGroup {
    pub fn create_travel_group(&self, ctx: &GraphQLContext, travel_group: NewTravelGroup) -> FieldResult<TravelGroup> {
        
        let conn = ctx.pool.get().expect("Unable to connect to DB");
        let res = diesel::insert_into(travel_groups::table)
            .values(travel_group)
            .get_result(&conn);

        graphql_translate(res)
    }
}

#[derive(Insertable, Debug, GraphQLInputObject)]
#[table_name = "travel_groups"]
pub struct NewTravelGroup {
    pub id: Uuid,
}

impl NewTravelGroup {
    pub fn new() -> Self {
        NewTravelGroup {
            id: Uuid::new_v4()
        }
    }
}
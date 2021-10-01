use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::{self, Insertable, Queryable};
use diesel::{RunQueryDsl, QueryDsl};
use uuid::Uuid;
use juniper::{FieldResult};

use crate::schema::*;
use crate::graphql::{graphql_translate};
use crate::GraphQLContext;
use super::{Trip, Person};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, PartialEq, PartialOrd, Identifiable)]
#[serde(rename_all= "snake_case")]
#[table_name = "travel_groups"]
/// People travelling together in a single voyage
/// May include several trips per person with distinct origins
/// and destinations.
/// Referenced through Person, Trip and links to voyage
pub struct TravelGroup {
    pub id: Uuid,
}

#[juniper::graphql_object(Context = GraphQLContext)]
impl TravelGroup {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn trips(&self, ctx: &GraphQLContext) -> Vec<Trip> {
        let conn = ctx.pool.get().expect("Unable to connect to DB");

        let res = trips::table.
            filter(trips::travel_group_id.eq(self.id))
            .order_by(trips::arrival_time)
            .order_by(trips::person_id)
            .load::<Trip>(&conn);

        res.unwrap()
    }

    pub fn people(&self, ctx: &GraphQLContext) -> Vec<Person> {
        let conn = ctx.pool.get().expect("Unable to connect to DB");

        let res = persons::table.
            filter(persons::travel_group_id.eq(self.id))
            .load::<Person>(&conn);

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
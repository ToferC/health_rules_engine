use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::{self, Insertable, PgConnection, Queryable};
use diesel::{RunQueryDsl, QueryDsl};
use uuid::Uuid;
use juniper::{FieldResult};

use crate::database::PostgresPool;
use crate::schema::*;
use crate::graphql::{graphql_translate};
use crate::GraphQLContext;
use super::{Trips};

type PG = diesel::pg::Pg;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, PartialEq, PartialOrd, Identifiable, Eq, Ord)]
#[serde(rename_all= "snake_case")]
#[table_name = "travel_groups"]
/// People travelling together
/// Referenced through Person, Trip and links to voyage
pub struct TravelGroups {
    pub id: Uuid,
}

#[juniper::graphql_object(Context = GraphQLContext)]
impl TravelGroups {
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

    pub fn all_travel_groups(ctx: &GraphQLContext) -> FieldResult<Vec<TravelGroups>> {
        
        let conn = ctx.pool.get().expect("Unable to connect to DB");
        let res = travel_groups::table.load::<TravelGroups>(&conn);

        graphql_translate(res)
    }

    pub fn travel_group_by_id(ctx: &GraphQLContext, id: Uuid) -> FieldResult<TravelGroups> {
        
        let conn = ctx.pool.get().expect("Unable to connect to DB");
        let res = travel_groups::table.filter(travel_groups::id.eq(&id))
            .first(&conn);

        graphql_translate(res)
    }

    pub fn create_travel_group(ctx: &GraphQLContext, travel_group: NewTravelGroup) -> FieldResult<TravelGroups> {
        
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
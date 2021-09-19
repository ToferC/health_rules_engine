use crate::{GraphQLContext, 
    database::PostgresPool, 
    errors::error_handler::CustomError
};

use diesel::{pg::PgConnection, RunQueryDsl};
use diesel::{QueryDsl, ExpressionMethods};
use juniper::http::GraphQLBatchRequest;
use juniper::{EmptySubscription, FieldError, FieldResult, RootNode};
use crate::schema::*;

use crate::models::{Trips, TripState, NewTrip, TravelGroup};
use uuid::Uuid;

pub struct Query;

#[juniper::graphql_object(Context =  GraphQLContext)]
impl Query {

    #[graphql(name = "allTrips")]
    pub fn all_trips(context: &GraphQLContext) -> FieldResult<Vec<Trips>> {
        let conn  = &context.pool.get().unwrap();

        Trips::all_trips(conn)
    }

    #[graphql(name = "travelGroups")]
    pub fn all_travel_groups(context: &GraphQLContext) -> FieldResult<Vec<TravelGroup>> {
        let conn = context.pool.get().expect("Unable to connect to db");

        let res = travel_groups::table.load::<TravelGroup>(&conn);

        graphql_translate(res)
    }

    #[graphql(name = "travelGroupByID")]
    pub fn travel_group_by_id(context: &GraphQLContext, id: Uuid) -> FieldResult<TravelGroup> {
        let conn = context.pool.get().expect("Unable to connect to db");
        let res = travel_groups::table
            .filter(travel_groups::id.eq(&id))
            .first(&conn);
        
        graphql_translate(res)

    }
}

pub struct Mutation;

#[juniper::graphql_object(Context = GraphQLContext)]
impl Mutation {
    #[graphql(name = "createTrip")]
    pub fn create_trip(
    context: &GraphQLContext,
    _input: String, // CreateTripInput
    ) -> FieldResult<Trips> {
        let conn  = &context.pool.get().unwrap();

        Trips::create_trip(conn, &NewTrip::default())
    }
}

pub fn graphql_translate<T>(res: Result<T, diesel::result::Error>) -> FieldResult<T> {
    match res {
        Ok(t) => Ok(t),
        Err(e) => Err(FieldError::from(e)),
    }
}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<GraphQLContext>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}

pub fn create_context(pg_pool: PostgresPool) -> GraphQLContext {
    GraphQLContext { pool: pg_pool }
}

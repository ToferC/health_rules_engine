use crate::{GraphQLContext, errors::error_handler::CustomError, models::NewTrip};
use diesel::pg::PgConnection;
use juniper::{EmptySubscription, FieldError, FieldResult, RootNode};

use super::models::{Trips, TripState};

pub struct Query;

#[juniper::graphql_object(Context =  GraphQLContext)]
impl Query {

    #[graphql(name = "allTrips")]
    pub fn all_trips(context: &GraphQLContext) -> FieldResult<Vec<Trips>> {
        let conn  = &context.pool.get().unwrap();

        Trips::all_trips(conn)
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

        Trips::create_trip(conn, NewTrip::default())
    }
}

pub fn graphql_translate<T>(res: Result<T, diesel::result::Error>) -> FieldResult<T> {
    match res {
        Ok(t) => Ok(t),
        Err(e) => Err(FieldError::from(e)),
    }
}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::default())
}

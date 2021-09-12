use crate::{GraphQLContext, errors::error_handler::CustomError};
use diesel::pg::PgConnection;
use juniper::{FieldResult, FieldError, RootNode};

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

pub fn graphql_translate<T>(res: Result<T, CustomError>) -> FieldResult<T> {
    match res {
        Ok(t) => Ok(t),
        Err(e) => Err(FieldError::from(e)),
    }
}

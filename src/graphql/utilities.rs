use std::sync::{Arc, Mutex};
use crate::{database::PostgresPool};

use async_graphql::*;

// use juniper::{EmptySubscription, FieldError, FieldResult, RootNode};

use crate::{GraphQLContext};
use crate::models::{Country, Place, Vaccine,};
use crate::graphql::{Query, Mutation};

pub fn graphql_translate<T>(res: Result<T, diesel::result::Error>) -> FieldResult<T> {
    match res {
        Ok(t) => Ok(t),
        Err(e) => Err(FieldError::from(e)),
    }
}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<GraphQLContext>>;

pub fn create_schema() -> async_graphql::Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}

pub fn create_context(pg_pool: PostgresPool) -> GraphQLContext {

    let conn = pg_pool.get().expect("Unable to connect to db");

    let countries = Arc::new(Mutex::new(Country::load_into_hash(&conn)));
    let places = Arc::new(Mutex::new(Place::load_into_hash(&conn)));
    let vaccines = Vaccine::load_into_hash(&conn);
    let identity = None;

    GraphQLContext { 
        pool: pg_pool,
        countries,
        places,
        vaccines,
        identity, 
    }
}
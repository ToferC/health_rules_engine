use std::sync::{Arc, Mutex};
use crate::{database::PostgresPool};

use async_graphql::*;

use crate::models::{Country, Place, Vaccine,};
use crate::graphql::{Query, Mutation};

pub fn graphql_translate<T>(res: Result<T, diesel::result::Error>) -> FieldResult<T> {
    match res {
        Ok(t) => Ok(t),
        Err(e) => Err(FieldError::from(e)),
    }
}

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn create_schema(pg_pool: PostgresPool) -> async_graphql::Schema<Query, Mutation, EmptySubscription> {
    
    let arc_conn = Arc::new(pg_pool.clone());

    let conn = pg_pool.get().expect("Unable to connect to db");

    let countries = Arc::new(Mutex::new(Country::load_into_hash(&conn)));
    let places = Arc::new(Mutex::new(Place::load_into_hash(&conn)));
    let vaccines = Vaccine::load_into_hash(&conn);
    let identity: Option<String> = None;
    
    Schema::build(Query, Mutation, EmptySubscription)
        .data(arc_conn)
        .data(countries)
        .data(places)
        .data(vaccines)
        .data(identity)
        .finish()
}
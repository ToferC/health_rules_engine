use crate::{ 
    database::PostgresPool, 
};

use diesel::{RunQueryDsl};
use diesel::{QueryDsl, ExpressionMethods};
use juniper::{EmptySubscription, FieldError, FieldResult, RootNode};
use crate::schema::*;

use crate::GraphQLContext;
use crate::models::{Country, NewTrip, Person, Place, QuarantinePlan,
    TravelGroup, Trips, Vaccination, Vaccine, CovidTest};
use uuid::Uuid;

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
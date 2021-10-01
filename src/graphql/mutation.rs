use crate::{ 
    database::PostgresPool, 
};

use diesel::{RunQueryDsl};
use diesel::{QueryDsl, ExpressionMethods};
use juniper::{EmptySubscription, FieldError, FieldResult, RootNode};
use crate::schema::*;

use crate::GraphQLContext;
use crate::models::{Country, NewTrip, Person, Place, QuarantinePlan,
    TravelGroup, Trip, Vaccination, Vaccine, CovidTest, TravelData, 
    TravelResponse, NewTravelResponse};
use uuid::Uuid;

pub struct Mutation;

#[graphql_object(Context = GraphQLContext)]
impl Mutation {

    #[graphql(name = "postTravelGroupData")]
    pub fn post_travel_group_data(
        context: &GraphQLContext,
        data: Vec<TravelData>,
    ) -> FieldResult<Vec<TravelResponse>> {

        let mut responses_to_cbsa: Vec<TravelResponse> = Vec::new();

        for traveller in data {
            let response = traveller.process(&context)?;
            responses_to_cbsa.push(response);
        }

        Ok(responses_to_cbsa)
    }

    pub fn ping(
        _context: &GraphQLContext,
        data: String,
    ) -> String {
        if data == "PING".to_string() {
            "PONG".to_string()
        } else {
            "WRONG".to_string()
        }
    }
}
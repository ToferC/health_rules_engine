use juniper::{EmptySubscription, FieldError, FieldResult};

use crate::GraphQLContext;
use crate::models::{Country, NewTrip, Person, Place, QuarantinePlan,
    TravelGroup, Trip, Vaccination, Vaccine, CovidTest, TravelData, 
    TravelResponse, NewTravelResponse};

pub struct Mutation;

#[graphql_object(Context = GraphQLContext)]
impl Mutation {

    #[graphql(name = "postTravelGroupData")]
    pub fn post_travel_group_data(
        context: &GraphQLContext,
        data: TravelData,
    ) -> FieldResult<TravelResponse> {

        let mut responses_to_cbsa: Vec<TravelResponse> = Vec::new();

        /*
        for traveller in data {
            responses_to_cbsa.push(response);
        }
        */
        let response = data.process(&context)?;
        
        Ok(response)
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
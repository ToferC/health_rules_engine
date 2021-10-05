use juniper::{EmptySubscription, FieldError, FieldResult};
use actix_identity::Identity;

use crate::{GraphQLContext, login};
use crate::models::{Country, CovidTest, LoginQuery, NewTravelResponse, NewTrip, Person, Place, QuarantinePlan, SlimUser, TravelData, TravelGroup, TravelResponse, Trip, Vaccination, Vaccine};

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

    pub fn login_user(
        context: &GraphQLContext,
        auth_data: LoginQuery,
    ) -> FieldResult<SlimUser> {
        
        login(&auth_data.email, &auth_data.password, &context)
            .and_then(|res| {
                let user_string =
                    serde_json::to_string(&res).map_err(|e| FieldError::new(
                        "Unable to login",
                        graphql_value!({ "internal_error": "Unable to log in"})))?;

            println!("user_string={}", user_string);
            //context.identity = Some(user_string);
            Ok(res)
        })
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
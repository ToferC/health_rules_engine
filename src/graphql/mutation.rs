use async_graphql::*;
use actix_identity::Identity;

use crate::{login};
use crate::models::{Country, CovidTest, LoginQuery, NewTravelResponse, NewTrip, Person, Place, QuarantinePlan, SlimUser, TravelData, TravelGroup, TravelResponse, Trip, Vaccination, Vaccine};

pub struct Mutation;

#[Object]
impl Mutation {

    #[graphql(name = "postTravelGroupData")]
    pub async fn post_travel_group_data(
        &self,
        context: &Context<'_>,
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

    pub async fn login_user(
        &self,
        context: &Context<'_>,
        auth_data: LoginQuery,
    ) -> FieldResult<SlimUser> {
        
        login(&auth_data.email, &auth_data.password, &context)
            .and_then(|res| {
                let user_string =
                    serde_json::to_string(&res).map_err(|e| e)?;

            println!("user_string={}", user_string);
            //context.identity = Some(user_string);
            Ok(res)
        })
    }

    pub async fn ping(
        &self,
        _context: &Context<'_>,
        data: String,
    ) -> String {
        if data == "PING".to_string() {
            "PONG".to_string()
        } else {
            "WRONG".to_string()
        }
    }
}
use async_graphql::*;
use actix_identity::Identity;
use uuid::Uuid;

use crate::{login};
use crate::models::{Country, CovidTest, LoginQuery, NewTravelResponse, NewTrip, Person, Place, QuarantinePlan, SlimUser, TravelData, TravelGroup, TravelResponse, Trip, Vaccination, Vaccine};

pub struct Mutation;

#[Object]
impl Mutation {

    #[graphql(name = "travelDataResponse")]
    /// Receives a Vec<TravelData> containing details from a group of travllers
    /// and returns a Vec<TravelResponse> containing public health direction for the BSO
    /// relating to entry to Canada for public health reasons and referrals to mandatory
    /// random testing. Also includes IDs for Person, Trip, QuarantinePlan
    /// for further mutations.
    pub async fn travel_data_response(
        &self,
        context: &Context<'_>,
        data: Vec<TravelData>,
    ) -> FieldResult<Vec<TravelResponse>> {

        let mut responses_to_cbsa: Vec<TravelResponse> = Vec::new();

        let travel_group_id = Uuid::new_v4();

        for traveller in data {
            let response = traveller.process(&context, travel_group_id)?.into();
            responses_to_cbsa.push(response);
        };
        
        
        Ok(responses_to_cbsa)
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
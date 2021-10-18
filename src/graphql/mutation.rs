use std::str::FromStr;

use async_graphql::*;
use async_graphql::guard::Guard;
use actix_identity::Identity;
use uuid::Uuid;

use crate::{login};
use crate::models::{Country, CovidTest, 
    LoginQuery, NewTravelResponse, 
    NewTrip, Person, Place, QuarantinePlan, 
    SlimUser, TravelData, TravelGroup, User,
    TravelResponse, Trip, Vaccination, Vaccine,
    hash_password, verify_password,
    create_token};
use crate::models::Role as AuthRole;
use crate::graphql::get_connection_from_context;

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

    pub async fn sign_in(
        &self,
        context: &Context<'_>,
        auth_data: LoginQuery,
    ) -> Result<String, Error> {

        let conn = get_connection_from_context(&context);

        let maybe_user = User::get_by_email(&auth_data.email, &conn).ok();

        if let Some(user) = maybe_user {
            if let Ok(matching) = verify_password(&user.hash.to_string(), &auth_data.password) {
                if matching {
                    let role = AuthRole::from_str(user.role.as_str())
                        .expect("Cannot convert &str to AuthRole");
                    return Ok(create_token(user.id.to_string(), role));
                }
            }
        }

        Err(Error::new("Can't authenticate a user"))
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
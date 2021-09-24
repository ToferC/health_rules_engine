use chrono::{Duration, prelude::*};
use serde::{Deserialize, Serialize};
use diesel::{self, Insertable, PgConnection, Queryable, ExpressionMethods};
use diesel::{RunQueryDsl, QueryDsl};
use uuid::Uuid;
use diesel_derive_enum::DbEnum;
use juniper::{FieldResult};

use crate::schema::*;
use crate::graphql::graphql_translate;
use crate::GraphQLContext;
use super::health_profile::PostalAddress;
use crate::models::{Place, Person};

/// Travel information for a TravelGroup
/// CBSA responsible, but important for public health surveillance
#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "trips"]
pub struct Trips {
    pub id: Uuid,
    pub trip_provider: String,
    // None for travel_identifier == private travel
    pub travel_identifier: Option<String>,
    pub booking_id: Option<String>,
    pub travel_mode: String,
    pub origin_place_id: Uuid,
    pub transit_point_place_ids: Vec<Uuid>,
    pub destination_place_id: Uuid,
    pub travel_intent: String,
    pub scheduled_departure_time: Option<NaiveDateTime>,
    pub scheduled_arrival_time: Option<NaiveDateTime>,
    pub departure_time: Option<NaiveDateTime>,
    pub arrival_time: Option<NaiveDateTime>,
    pub trip_state: String,
    pub travel_group_id: Uuid,
    pub person_id: Uuid,
}

#[graphql_object(Context = GraphQLContext)]
impl Trips {
    pub fn id(&self) -> FieldResult<Uuid> {
        Ok(self.id)
    }

    pub fn trip_provider(&self) -> FieldResult<String> {
        Ok(self.trip_provider.to_owned())
    }

    pub fn travel_mode(&self) -> FieldResult<String> {
        Ok(self.travel_mode.to_owned())
    }

    pub fn travel_identifier(&self) -> FieldResult<String> {
        match &self.travel_identifier {
            Some(i) => Ok(i.to_owned()),
            None => Ok("None".to_string()),
        }
    }

    pub fn booking_id(&self) -> FieldResult<String> {
        match &self.booking_id {
            Some(i) => Ok(i.to_owned()),
            None => Ok("None".to_string()),
        }
    }

    pub fn scheduled_departure_time(&self) -> FieldResult<String> {
        match self.scheduled_arrival_time {
            Some(t) => Ok(t.format("%Y-%m-%d %H:%M:%S").to_string()),
            None => Ok("NA".to_string()),
        } 
    }

    pub fn scheduled_arrival_time(&self) -> FieldResult<String> {
        match self.scheduled_departure_time {
            Some(t) => Ok(t.format("%Y-%m-%d %H:%M:%S").to_string()),
            None => Ok("NA".to_string()),
        } 
    }

    pub fn departure_time(&self) -> FieldResult<String> {
        match self.arrival_time {
            Some(t) => Ok(t.format("%Y-%m-%d %H:%M:%S").to_string()),
            None => Ok("NA".to_string()),
        } 
    }

    pub fn arrival_time(&self) -> FieldResult<String> {
        match self.departure_time {
            Some(t) => Ok(t.format("%Y-%m-%d %H:%M:%S").to_string()),
            None => Ok("NA".to_string()),
        } 
    }

    pub fn travel_intent(&self) -> FieldResult<String> {
        Ok(self.travel_intent.to_owned())
    }

    pub fn trip_state(&self) -> FieldResult<String> {
        Ok(self.trip_state.to_owned())
    }

    pub fn person(&self, context: &GraphQLContext) -> FieldResult<Person> {

        let conn = context.pool.get().expect("Unable to connect to DB");

        let res = persons::table.
            filter(persons::id.eq(self.person_id))
            .first(&conn);

        graphql_translate(res)
    }

    pub fn origin(&self, context: &GraphQLContext) -> FieldResult<Place> {

        let conn = context.pool.get().expect("Unable to connect to DB");

        let res = places::table.
            filter(places::id.eq(self.origin_place_id))
            .first(&conn);

        graphql_translate(res)
    }

    pub fn destination(&self, context: &GraphQLContext) -> FieldResult<Place> {

        let conn = context.pool.get().expect("Unable to connect to DB");

        let res = places::table.
            filter(places::id.eq(self.destination_place_id))
            .first(&conn);

        graphql_translate(res)
    }
}

// Non Graphql
impl Trips {
    pub fn create_trip(conn: &PgConnection, trip: &NewTrip) -> FieldResult<Trips> {
        let res = diesel::insert_into(trips::table)
            .values(trip)
            .get_result(conn);

        graphql_translate(res)
    }
}

#[derive(Insertable, Debug, GraphQLInputObject)]
#[table_name = "trips"]
pub struct NewTrip {
    pub trip_provider: String,
    // None for travel_identifier == private travel
    pub travel_identifier: Option<String>,
    pub booking_id: Option<String>,
    pub travel_mode: String,
    pub origin_place_id: Uuid,
    pub transit_point_place_ids: Vec<Uuid>,
    pub destination_place_id: Uuid,
    pub travel_intent: String,
    pub scheduled_departure_time: Option<NaiveDateTime>,
    pub scheduled_arrival_time: Option<NaiveDateTime>,
    pub departure_time: Option<NaiveDateTime>,
    pub arrival_time: Option<NaiveDateTime>,
    pub trip_state: String,
    pub travel_group_id: Uuid,
    pub person_id: Uuid,
}

impl<'a> NewTrip {
    pub fn default() -> Self {

        let depart: NaiveDateTime = Utc::now().naive_utc() - Duration::hours(8);
        let arrive: NaiveDateTime = Utc::now().naive_utc() + Duration::hours(4);

        NewTrip { 
            trip_provider: "Air Canada".to_string(), 
            travel_identifier: Some("ADX-Q6)Y".to_string()), 
            booking_id: Some("678326432632".to_string()), 
            travel_mode: "AIR".to_string(), 
            origin_place_id: Uuid::new_v4(),
            transit_point_place_ids: vec![Uuid::new_v4()],
            destination_place_id: Uuid::new_v4(), 
            
            travel_intent: "Entry".to_string(), 
            scheduled_departure_time: Some(depart), 
            scheduled_arrival_time: Some(arrive + Duration::hours(4)), 
            departure_time: Some(depart), 
            arrival_time: Some(arrive), 
            trip_state: "planned".to_string(),
            travel_group_id: Uuid::new_v4(),
            person_id: Uuid::new_v4(),
        }
    }

    pub fn new(
        travel_group_id: &Uuid,
        person_id: &Uuid,
        origin_place_id: &Uuid,
        destination_place_id: &Uuid
    ) -> Self 
    {
        let depart: NaiveDateTime = Utc::now().naive_utc() - Duration::days(1);
        let arrive: NaiveDateTime = Utc::now().naive_utc() + Duration::days(1);

        NewTrip { 
            trip_provider: "Air Canada".to_string(), 
            travel_identifier: Some("ADX-Q6)Y".to_string()), 
            booking_id: Some("678326432632".to_string()), 
            travel_mode: "AIR".to_string(), 
            origin_place_id: origin_place_id.to_owned(), 
            transit_point_place_ids: Vec::new(), 
            
            destination_place_id: destination_place_id.to_owned(), 
            
            travel_intent: "Entry".to_string(), 
            scheduled_departure_time: Some(depart), 
            scheduled_arrival_time: Some(arrive + Duration::hours(4)), 
            departure_time: Some(depart), 
            arrival_time: Some(arrive), 
            trip_state: "planned".to_string(),
            travel_group_id: travel_group_id.to_owned(),
            person_id: person_id.to_owned(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, DbEnum)]
pub enum TripState {
    Planned,
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum TravelIntent {
    Entry,
    Exit,
    Transit,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// Should get this info from an API
/// Could be a struct with company info, contact info, API key, etc.
pub struct TravelProvider {
    id: Uuid,
    name: String,
    description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum TravelMode {
    // Strings annotate the types of travel
    // I.e., "Rail", "Bus", "Private Vehicle", "Charter"
    Air(String),
    Sea(String),
    Land(String),
}
use std::sync::Arc;

use diesel::{RunQueryDsl};
use diesel::{QueryDsl, ExpressionMethods};
use crate::schema::*;

use async_graphql::*;

use crate::database::POOL;
use crate::models::{Person, QuarantinePlan,
    TravelGroup, Trip, Vaccination, CovidTest};
use uuid::Uuid;
use crate::graphql::graphql_translate;

pub struct Query;

#[Object]
impl Query {

    #[graphql(name = "allTrips")]
    pub async fn all_trips(
        &self, 
        context: &Context<'_>,) -> FieldResult<Vec<Trip>> {
        let conn  = &context.data::<Arc<POOL>>()?.get().unwrap();

        let res = trips::table
            .order(trips::arrival_time)
            .load::<Trip>(conn);

        graphql_translate(res)
    }

    #[graphql(name = "tripById")]
    pub async fn trip_by_id(
        &self, 
        context: &Context<'_>,
        id: Uuid
    ) -> FieldResult<Trip> {

        let conn = context.data::<Arc<POOL>>()?.get().expect("Unable to connect to DB");

        let res = trips::table.filter(trips::id.eq(id))
            .first(&conn);

        graphql_translate(res)
    }

    #[graphql(name = "travelGroups")]
    pub async fn all_travel_groups(
        &self, 
        context: &Context<'_>,
    ) -> FieldResult<Vec<TravelGroup>> {
        let conn = context.data::<Arc<POOL>>()?.get().expect("Unable to connect to db");

        let res = travel_groups::table.load::<TravelGroup>(&conn);

        graphql_translate(res)
    }

    
    #[graphql(name = "travelGroupByID")]
    pub async fn travel_group_by_id(
        &self, 
        context: &Context<'_>,
        id: Uuid
    ) -> FieldResult<TravelGroup> {
        let conn = context.data::<Arc<POOL>>()?.get().expect("Unable to connect to db");
        let res = travel_groups::table
        .filter(travel_groups::id.eq(&id))
        .first(&conn);
        
        graphql_translate(res)
    }

    #[graphql(name = "allPeople")]
    pub async fn all_people(&self, context: &Context<'_>) -> FieldResult<Vec<Person>> {
        let conn = context.data::<Arc<POOL>>()?.get().expect("Unable to connect to db");

        let res = persons::table.load::<Person>(&conn);

        graphql_translate(res)
    }

    #[graphql(name = "allVaccinations")]
    pub async fn all_vaccinations(&self, context: &Context<'_>) -> FieldResult<Vec<Vaccination>> {
        let conn = context.data::<Arc<POOL>>()?.get().expect("Unable to connect to db");

        let res = vaccinations::table.load::<Vaccination>(&conn);

        graphql_translate(res)
    }

    #[graphql(name = "allQuarantinePlans")]
    pub async fn all_quarantine_plans(&self, context: &Context<'_>) -> FieldResult<Vec<QuarantinePlan>> {
        let conn = context.data::<Arc<POOL>>()?.get().expect("Unable to connect to db");

        let res = quarantine_plans::table.load::<QuarantinePlan>(&conn);

        graphql_translate(res)
    }

    #[graphql(name = "allCovidTestResults")]
    pub async fn all_covid_test_results(&self, context: &Context<'_>) -> FieldResult<Vec<CovidTest>> {
        let conn = context.data::<Arc<POOL>>()?.get().expect("Unable to connect to db");

        let res = covid_tests::table.load::<CovidTest>(&conn);

        graphql_translate(res)
    }
}
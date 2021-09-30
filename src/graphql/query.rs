use diesel::{RunQueryDsl};
use diesel::{QueryDsl, ExpressionMethods};
use juniper::{FieldResult};
use crate::schema::*;

use crate::GraphQLContext;
use crate::models::{Person, QuarantinePlan,
    TravelGroup, Trips, Vaccination, CovidTest};
use uuid::Uuid;
use crate::graphql::graphql_translate;

pub struct Query;

#[juniper::graphql_object(Context =  GraphQLContext)]
impl Query {

    #[graphql(name = "allTrips")]
    pub fn all_trips(context: &GraphQLContext) -> FieldResult<Vec<Trips>> {
        let conn  = &context.pool.get().unwrap();

        let res = trips::table
            .load::<Trips>(conn);

        graphql_translate(res)
    }

    #[graphql(name = "tripById")]
    pub fn trip_by_id(context: &GraphQLContext, id: Uuid) -> FieldResult<Trips> {

        let conn = context.pool.get().expect("Unable to connect to DB");

        let res = trips::table.filter(trips::id.eq(id))
            .first(&conn);

        graphql_translate(res)
    }

    #[graphql(name = "travelGroups")]
    pub fn all_travel_groups(context: &GraphQLContext) -> FieldResult<Vec<TravelGroup>> {
        let conn = context.pool.get().expect("Unable to connect to db");

        let res = travel_groups::table.load::<TravelGroup>(&conn);

        graphql_translate(res)
    }

    
    #[graphql(name = "travelGroupByID")]
    pub fn travel_group_by_id(context: &GraphQLContext, id: Uuid) -> FieldResult<TravelGroup> {
        let conn = context.pool.get().expect("Unable to connect to db");
        let res = travel_groups::table
        .filter(travel_groups::id.eq(&id))
        .first(&conn);
        
        graphql_translate(res)
    }

    #[graphql(name = "allPeople")]
    pub fn all_people(context: &GraphQLContext) -> FieldResult<Vec<Person>> {
        let conn = context.pool.get().expect("Unable to connect to db");

        let res = persons::table.load::<Person>(&conn);

        graphql_translate(res)
    }

    #[graphql(name = "allVaccinations")]
    pub fn all_vaccinations(context: &GraphQLContext) -> FieldResult<Vec<Vaccination>> {
        let conn = context.pool.get().expect("Unable to connect to db");

        let res = vaccinations::table.load::<Vaccination>(&conn);

        graphql_translate(res)
    }

    #[graphql(name = "allQuarantinePlans")]
    pub fn all_quarantine_plans(context: &GraphQLContext) -> FieldResult<Vec<QuarantinePlan>> {
        let conn = context.pool.get().expect("Unable to connect to db");

        let res = quarantine_plans::table.load::<QuarantinePlan>(&conn);

        graphql_translate(res)
    }

    #[graphql(name = "allCovidTestResults")]
    pub fn all_covid_test_results(context: &GraphQLContext) -> FieldResult<Vec<CovidTest>> {
        let conn = context.pool.get().expect("Unable to connect to db");

        let res = covid_tests::table.load::<CovidTest>(&conn);

        graphql_translate(res)
    }
}
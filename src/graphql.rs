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

pub fn graphql_translate<T>(res: Result<T, diesel::result::Error>) -> FieldResult<T> {
    match res {
        Ok(t) => Ok(t),
        Err(e) => Err(FieldError::from(e)),
    }
}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<GraphQLContext>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}

pub fn create_context(pg_pool: PostgresPool) -> GraphQLContext {

    let conn = pg_pool.get().expect("Unable to connect to db");

    let countries = Country::load_into_hash(&conn);
    let places = Place::load_into_hash(&conn);
    let vaccines = Vaccine::load_into_hash(&conn);

    GraphQLContext { 
        pool: pg_pool,
        countries,
        places,
        vaccines,    
    }
}

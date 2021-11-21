use diesel::{RunQueryDsl};
use diesel::{QueryDsl, ExpressionMethods};
use crate::schema::*;

use async_graphql::*;

use crate::models::{Person, QuarantinePlan, User,
    TravelGroup, Trip, Vaccination, CovidTest};
use uuid::Uuid;

use crate::graphql::{graphql_translate, get_connection_from_context};
use crate::common_utils::{RoleGuard, is_admin, Role as AuthRole};

pub struct Query;

#[Object]
impl Query {

    #[graphql(name = "allTrips")]
    /// Returns a vector of all trips ordered by arrival time
    pub async fn all_trips(
        &self, 
        context: &Context<'_>,) -> FieldResult<Vec<Trip>> {

        let conn = get_connection_from_context(context);

        let res = trips::table
            .order(trips::arrival_time)
            .load::<Trip>(&conn);

        graphql_translate(res)
    }

    #[graphql(name = "getTrips")]
    /// Accepts argument of "count" and returns a vector of {count} trips ordered by
    /// arrival time.
    pub async fn get_trips(
        &self, 
        context: &Context<'_>,
        count: i64,
    ) -> FieldResult<Vec<Trip>> {

        let conn = get_connection_from_context(context);

        let res = trips::table
            .order(trips::arrival_time)
            .limit(count)
            .load::<Trip>(&conn);

        graphql_translate(res)
    }

    #[graphql(name = "tripById")]
    pub async fn trip_by_id(
        &self, 
        context: &Context<'_>,
        id: Uuid
    ) -> FieldResult<Trip> {

        let conn = get_connection_from_context(context);

        let res = trips::table.filter(trips::id.eq(id))
            .first(&conn);

        graphql_translate(res)
    }

    /// Travel Groups
    #[graphql(name = "allTravelGroups")]
    /// Returns a vector of all travel groups
    pub async fn all_travel_groups(
        &self, 
        context: &Context<'_>,
    ) -> FieldResult<Vec<TravelGroup>> {
        let conn = get_connection_from_context(context);

        let res = travel_groups::table.load::<TravelGroup>(&conn);

        graphql_translate(res)
    }

    
    #[graphql(name = "travelGroupByID")]
    /// Returns a specific travel group by its UUID
    pub async fn travel_group_by_id(
        &self, 
        context: &Context<'_>,
        id: Uuid
    ) -> FieldResult<TravelGroup> {
        let conn = get_connection_from_context(context);

        let res = travel_groups::table
        .filter(travel_groups::id.eq(&id))
        .first(&conn);
        
        graphql_translate(res)
    }

    #[graphql(name = "allPeople")]
    /// Returns a vector of all people
    pub async fn all_people(&self, context: &Context<'_>) -> FieldResult<Vec<Person>> {
        let conn = get_connection_from_context(context);

        let res = persons::table.load::<Person>(&conn);

        graphql_translate(res)
    }

    #[graphql(name = "getPeople")]
    /// Accepts an argument of "count" and returns a vector of {count} people
    pub async fn get_people(&self, context: &Context<'_>, count: i64) -> FieldResult<Vec<Person>> {
        let conn = get_connection_from_context(context);

        let res = persons::table
            .limit(count)
            .load::<Person>(&conn);

        graphql_translate(res)
    }

    #[graphql(name = "allVaccinations")]
    /// Returns a vector of all vaccination histories
    pub async fn all_vaccinations(&self, context: &Context<'_>) -> FieldResult<Vec<Vaccination>> {
        let conn = get_connection_from_context(context);

        let res = vaccinations::table.load::<Vaccination>(&conn);

        graphql_translate(res)
    }

    #[graphql(name = "getVaccinations")]
    /// Accepts argument "count" and returns a vector of {count} vaccination histories
    pub async fn get_vaccinations(&self, context: &Context<'_>, count: i64) -> FieldResult<Vec<Vaccination>> {
        let conn = get_connection_from_context(context);

        let res = vaccinations::table
            .limit(count)
            .load::<Vaccination>(&conn);

        graphql_translate(res)
    }

    #[graphql(name = "allQuarantinePlans")]
    /// Returns a vector of all quarantine plans
    pub async fn all_quarantine_plans(&self, context: &Context<'_>) -> FieldResult<Vec<QuarantinePlan>> {
        let conn = get_connection_from_context(context);

        let res = quarantine_plans::table.load::<QuarantinePlan>(&conn);

        graphql_translate(res)
    }

    #[graphql(name = "getQuarantinePlans")]
    /// Accepts argument "count" and returns a vector of {count} quarantine plans
    pub async fn get_quarantine_plans(&self, context: &Context<'_>, count: i64) -> FieldResult<Vec<QuarantinePlan>> {
        let conn = get_connection_from_context(context);

        let res = quarantine_plans::table
            .limit(count)
            .load::<QuarantinePlan>(&conn);

        graphql_translate(res)
    }

    #[graphql(name = "allCovidTestResults")]
    /// Returns a vector of all covid test results
    pub async fn all_covid_test_results(&self, context: &Context<'_>) -> FieldResult<Vec<CovidTest>> {
        let conn = get_connection_from_context(context);

        let res = covid_tests::table.load::<CovidTest>(&conn);

        graphql_translate(res)
    }

    #[graphql(name = "getCovidTestResults")]
    /// Accepts argument "count" and returns a vector of {count} covid test results
    pub async fn get_covid_test_results(&self, context: &Context<'_>, count: i64) -> FieldResult<Vec<CovidTest>> {
        let conn = get_connection_from_context(context);

        let res = covid_tests::table
            .limit(count)
            .load::<CovidTest>(&conn);

        graphql_translate(res)
    }

    #[graphql(
        name = "allUsers",
        guard = "RoleGuard::new(AuthRole::Admin)",
        visible = "is_admin",
    )]
    /// Returns a vector of all users
    pub async fn all_users(&self, context: &Context<'_>) -> FieldResult<Vec<User>> {
        let conn = get_connection_from_context(context);

        let res = users::table.load::<User>(&conn);

        graphql_translate(res)
    }

    #[graphql(
        name = "getUserByEmail",
        guard = "RoleGuard::new(AuthRole::Admin)",
        visible = "is_admin",
    )]
    /// Returns a vector of all users
    pub async fn get_user_by_email(&self, context: &Context<'_>, email: String) -> FieldResult<User> {
        let conn = get_connection_from_context(context);

        let res = User::get_by_email(&email, &conn);

        res
    }

    #[graphql(
        name = "getUserById",
        guard = "RoleGuard::new(AuthRole::Admin)",
        visible = "is_admin",
    )]
    /// Returns a vector of all users
    pub async fn get_user_by_id(&self, context: &Context<'_>, id: Uuid) -> FieldResult<User> {
        let conn = get_connection_from_context(context);

        let res = User::get_by_id(&id, &conn);

        res
    }
}
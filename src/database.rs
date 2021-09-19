
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::prelude::*;
use diesel::result::Error;
use lazy_static::lazy_static;
use r2d2::{self, PooledConnection};
use std::env;

use crate::errors::error_handler::CustomError;

pub type PostgresPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

use crate::models::{NewTrip, TravelGroup, NewTravelGroup, Trips, NewPerson, Person};
use crate::GraphQLContext;
use crate::schema::*;

#[macro_use]
embed_migrations!();

lazy_static! {
    pub static ref POOL: PostgresPool = {
        let db_url = env::var("DATABASE_URL").expect("Database url not set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        PostgresPool::new(manager).expect("Failed to create DB Pool")
    };
}

pub fn init() {
    lazy_static::initialize(&POOL);
    let conn = connection().expect("Failed to get DB connection");
    embedded_migrations::run(&conn).unwrap();

    println!("Adding Demo Data");
    populate_db_with_demo_data(&conn);
}

pub fn connection() -> Result<DbConnection, CustomError> {
    POOL.get()
        .map_err(|e| CustomError::new(500, format!("Failed getting db connection: {}", e)))
}

pub fn populate_db_with_demo_data(conn: &PgConnection) {
    let tg = crate::models::NewTravelGroup::new();

    let res: Result<TravelGroup, Error> = diesel::insert_into(travel_groups::table)
            .values(&tg)
            .get_result(conn);

    let travel_group = res.unwrap();

    for i in 0..4 {
        let person = NewPerson::new();

        let created_p = Person::create(conn, &person).expect("Unable to create person");

        let nt = NewTrip::new(&travel_group.id, &created_p.id);

        Trips::create_trip(conn, &nt);
    }

}
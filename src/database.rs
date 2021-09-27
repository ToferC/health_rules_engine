
use std::{io::stdin};
use diesel::dsl::count;
use chrono::prelude::*;
use chrono::Duration;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::prelude::*;
use diesel::result::Error;
use lazy_static::lazy_static;
use r2d2::{self, PooledConnection};
use rand::Rng;
use std::env;
use uuid::Uuid;
use rand::{thread_rng, seq::SliceRandom};

use crate::errors::error_handler::CustomError;

pub type PostgresPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

use crate::models::NewCovidTest;
use crate::models::NewQuarantinePlan;
use crate::models::QuarantinePlan;
use crate::models::{Country, NewCountry, NewPerson, NewPlace, 
    NewPublicHealthProfile, NewTravelGroup, NewTrip, NewVaccination, 
    NewVaccine, Person, Place, PublicHealthProfile, TravelGroup, 
    Trips, Vaccine, Vaccination, CovidTest};

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

    
    println!("Would you like to add demo data? (yes/no)");
    
    let mut response = String::new();
    stdin().read_line(&mut response).expect("Unable to read input");
    
    if response.trim() == "yes" || response.trim() == "y" {
        println!("Adding Demo Data");
        populate_db_with_demo_data(&conn);
    };
    
    println!("Database and connection initialized");

}

pub fn connection() -> Result<DbConnection, CustomError> {
    POOL.get()
        .map_err(|e| CustomError::new(500, format!("Failed getting db connection: {}", e)))
}

pub fn populate_db_with_demo_data(conn: &PgConnection) {

    // Set up countries
    let mut new_countries: Vec<NewCountry> = Vec::new();

    new_countries.push(NewCountry::new("United Kingdom".to_string(), 0.05));
    new_countries.push(NewCountry::new("Canada".to_string(), 0.03));
    new_countries.push(NewCountry::new("Singapore".to_string(), 0.02));
    new_countries.push(NewCountry::new("USA".to_string(), 0.04));
    new_countries.push(NewCountry::new("France".to_string(), 0.03));
    new_countries.push(NewCountry::new("Brazil".to_string(), 0.06));
   
    let mut countries: Vec<Country> = Vec::new();

    for np in new_countries {
        let c = Country::create(conn, &np).unwrap();
        countries.push(c);
    };

    // Set up places
    let mut new_places:Vec<NewPlace> = Vec::new();
    new_places.push(NewPlace::new("London".to_string(), countries[0].id));
    new_places.push(NewPlace::new("Singapore".to_string(), countries[2].id));
    new_places.push(NewPlace::new("Florida".to_string(), countries[3].id));
    new_places.push(NewPlace::new("Paris".to_string(), countries[4].id));
    new_places.push(NewPlace::new("Chicago".to_string(), countries[3].id));
    new_places.push(NewPlace::new("Rio".to_string(), countries[5].id));
    new_places.push(NewPlace::new("New York".to_string(), countries[3].id));
    new_places.push(NewPlace::new("Ottawa".to_string(), countries[1].id));
    new_places.push(NewPlace::new("Montreal".to_string(), countries[1].id));
    new_places.push(NewPlace::new("Vancouver".to_string(), countries[1].id));
    new_places.push(NewPlace::new("Calgary".to_string(), countries[1].id));
    new_places.push(NewPlace::new("Toronto".to_string(), countries[1].id));

    let mut origins: Vec<Place> = Vec::new();
    let mut destinations: Vec<Place> = Vec::new();

    for np in new_places {
        let p = Place::create(conn, &np).unwrap();

        if p.country_id != countries[1].id {
            origins.push(p);
        } else {
            destinations.push(p);
        }
    };

    // Set up RNG
    let mut rng = thread_rng();

    // Add Vaccines

    let mut new_vaccines = Vec::new();

    let approved_on: NaiveDateTime = Utc.ymd(2021, 09, 21).and_hms(1, 1, 1).naive_utc();

    let mut vaccines: Vec<Vaccine> = Vec::new();

    new_vaccines.push(
        NewVaccine::new(
            "Comirnaty".to_string(),
            "Phizer".to_string(),
            "mRNA".to_string(),
            2,
            true,
            approved_on,
            "XXX YYY".to_string()
    ));

    new_vaccines.push(
        NewVaccine::new(
            "SpikeVax".to_string(),
            "Moderna".to_string(),
            "mRNA".to_string(),
            2,
            true,
            approved_on,
            "XXX YYY".to_string()
    ));

    new_vaccines.push(
        NewVaccine::new(
            "Vaxzeria".to_string(),
            "AstraZeneca".to_string(),
            "Viral Vector-based".to_string(),
            2,
            true,
            approved_on,
            "XXX YYY".to_string()
    ));

    new_vaccines.push(
        NewVaccine::new(
            "Jannsen".to_string(),
            "Johnson & Johnson".to_string(),
            "mRNA".to_string(),
            1,
            true,
            approved_on,
            "XXX YYY".to_string()
    ));

    for v in new_vaccines {
        let res = Vaccine::create(conn, &v).unwrap();
        vaccines.push(res);
    }

    // Populate with fake population data

    for i in 0..100 {

        let tg = crate::models::NewTravelGroup::new();

        let res: Result<TravelGroup, Error> = diesel::insert_into(travel_groups::table)
                .values(&tg)
                .get_result(conn);

        let travel_group = res.unwrap();

        for i in 0..4 {

            let country = countries.choose(&mut rng).unwrap();

            // Create person
            let person = NewPerson::fake(
                country.id
            );

            let created_p = Person::create(conn, &person).expect("Unable to create person");
                
            // Create trip
            let origin  = origins.choose(&mut rng).unwrap();
            let destination = destinations.choose(&mut rng).unwrap();
            
            let nt = NewTrip::new(
                &travel_group.id, 
                &created_p.id, 
                &origin.id, 
                &destination.id
            );
            
            Trips::create_trip(conn, &nt);

            // Create public health profile
            let profile = NewPublicHealthProfile::new(
                created_p.id.to_owned(), 
                Uuid::new_v4().to_string(),
            );

            let created_ph_profile = PublicHealthProfile::create(conn, &profile).unwrap();

            // Create vaccinations
            for i in 0..2 {
                let new_vaccination = NewVaccination::new(
                    vaccines.choose(&mut rng).unwrap().id, 
                    "local pharmacy".to_string(), 
                    origin.id, 
                    Utc::now().naive_utc() - Duration::days(rng.gen_range(1..90)), 
                    created_ph_profile.id,
                );

                Vaccination::create(conn, &new_vaccination).unwrap();
            }

            // Create COVID Test
            let test_result = rng.gen_bool(country.risk_rate);

            let new_test = NewCovidTest::new(
                created_ph_profile.id, 
                "Test-X01".to_string(), 
                "molecular".to_string(), 
                Utc::now().naive_utc() - Duration::days(rng.gen_range(1..14)), 
                test_result);

            CovidTest::create(conn, &new_test);

            // Create quarantine plan

            let new_qp = NewQuarantinePlan::new(
                created_ph_profile.id,
                Utc::now().naive_utc() - Duration::days(rng.gen_range(1..14)),
                false,
                false,
                "Local Hotel Address".to_string(),
                false,
            );

            let r = QuarantinePlan::create(conn, &new_qp).unwrap();

            println!("{:?}", &r);

            println!("Demo data insert complete");

        }
    }

}
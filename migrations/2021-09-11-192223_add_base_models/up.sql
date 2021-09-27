-- Your SQL goes here

CREATE TYPE access_level_enum AS ENUM (
    'adminstrator',
    'analyst',
    'employee',
    'research',
    'open'
);

CREATE TABLE IF NOT EXISTS users (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    user_instance_uid UUID NOT NULL,
    email VARCHAR(128) UNIQUE NOT NULL,
    access_level VARCHAR NOT NULL,
    created_on TIMESTAMP NOT NULL,
    access_key VARCHAR(256) NOT NULL,
    approved_by_user_uid UUID
);

CREATE TABLE IF NOT EXISTS travel_groups (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS places (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    place_name VARCHAR NOT NULL,
    country_id UUID NOT NULL
);

CREATE TABLE IF NOT EXISTS countries (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    country_name VARCHAR NOT NULL
);

CREATE TYPE trip_state_enum AS ENUM ('planned', 'in_progress', 'completed', 'cancelled');

CREATE TABLE IF NOT EXISTS trips (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    trip_provider VARCHAR NOT NULL,
    travel_identifier VARCHAR,
    booking_id VARCHAR,
    travel_mode VARCHAR NOT NULL,
    origin_place_id UUID NOT NULL,
    transit_point_place_ids UUID[] NOT NULL,
    destination_place_id UUID NOT NULL,
    travel_intent VARCHAR NOT NULL,
    scheduled_departure_time TIMESTAMP,
    scheduled_arrival_time TIMESTAMP,
    departure_time TIMESTAMP,
    arrival_time TIMESTAMP,
    trip_state VARCHAR(64) NOT NULL,
    travel_group_id UUID NOT NULL,
    person_id UUID NOT NULL
);

CREATE TABLE IF NOT EXISTS persons (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    birth_date TIMESTAMP NOT NULL,
    travel_document_issuer_id UUID NOT NULL,
    approved_access_level VARCHAR NOT NULL,
    approved_access_granularity VARCHAR NOT NULL,
    travel_document_id UUID NOT NULL
);

CREATE TABLE IF NOT EXISTS public_health_profiles (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    person_id UUID NOT NULL,
    smart_healthcard_pk VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS vaccinations (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    vaccine_id UUID NOT NULL,
    dose_provider VARCHAR NOT NULL,
    location_provided_id UUID NOT NULL,
    provided_on TIMESTAMP NOT NULL,
    public_health_profile_id UUID NOT NULL
);

CREATE TABLE IF NOT EXISTS vaccines (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    vaccine_name VARCHAR NOT NULL,
    manufacturer VARCHAR NOT NULL,
    vaccine_type VARCHAR NOT NULL,
    required_doses INT NOT NULL,
    approved bool NOT NULL,
    approved_on TIMESTAMP NOT NULL,
    details TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS quarantine_plans (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    public_health_profile_id UUID NOT NULL,
    date_created TIMESTAMP NOT NULL,
    quarantine_required bool NOT NULL,
    confirmation_no_vulnerable bool NOT NULL,
    postal_address_id VARCHAR NOT NULL,
    active bool NOT NULL
);

CREATE TABLE IF NOT EXISTS check_in_results (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    quarantine_plan_id UUID NOT NULL,
    verifier_id UUID NOT NULL,
    date_time TIMESTAMP NOT NULL,
    check_in_complete bool NOT NULL
);

CREATE TABLE IF NOT EXISTS covid_tests (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    public_health_profile_id UUID NOT NULL,
    test_name VARCHAR NOT NULL,
    test_type VARCHAR NOT NULL,
    date_taken TIMESTAMP NOT NULL,
    test_result bool NOT NULL
);

CREATE TABLE IF NOT EXISTS postal_addresses (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    street_address VARCHAR NOT NULL,
    address_locality_id UUID NOT NULL,
    address_region VARCHAR NOT NULL,
    address_country_id UUID NOT NULL,
    postal_code VARCHAR NOT NULL,
    lattitude FLOAT NOT NULL,
    longitude FLOAT NOT NULL,
    additional_info TEXT
);
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
    access_level access_level_enum NOT NULL,
    created_on TIMESTAMP NOT NULL,
    access_key VARCHAR(256) NOT NULL,
    approved_by_user_uid UUID
);

CREATE TABLE IF NOT EXISTS travel_groups (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS places (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    place_name VARCHAR NOT NULL
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
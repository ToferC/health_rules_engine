-- Your SQL goes here

CREATE TYPE access_level_enum AS ENUM (
    'adminstrator',
    'analyst',
    'employee',
    'research',
    'open'
);

CREATE TABLE users (
    uid UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    user_instance_uid UUID NOT NULL,
    email VARCHAR(128) UNIQUE NOT NULL,
    access_level access_level_enum NOT NULL,
    created_on TIMESTAMP NOT NULL,
    access_key VARCHAR(256) NOT NULL,
    approved_by_user_uid UUID
);

CREATE TYPE trip_state_enum AS ENUM ('planned', 'in_progress', 'completed', 'cancelled');

CREATE TABLE IF NOT EXISTS trips (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    trip_provider VARCHAR NOT NULL,
    travel_identifier VARCHAR,
    booking_id VARCHAR,
    travel_mode VARCHAR NOT NULL,
    origin VARCHAR NOT NULL,
    transit_points TEXT[],
    destination VARCHAR NOT NULL,
    travel_intent VARCHAR NOT NULL,
    scheduled_departure_time TIMESTAMP,
    scheduled_arrival_time TIMESTAMP,
    departure_time TIMESTAMP,
    arrival_time TIMESTAMP,
    trip_state VARCHAR(64) NOT NULL
);
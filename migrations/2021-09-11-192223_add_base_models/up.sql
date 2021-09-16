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

CREATE TABLE IF NOT EXISTS travel_groups (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY
);

INSERT INTO travel_groups VALUES (
    gen_random_uuid()
);

CREATE TYPE trip_state_enum AS ENUM ('planned', 'in_progress', 'completed', 'cancelled');

CREATE TABLE IF NOT EXISTS trips (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    trip_provider VARCHAR NOT NULL,
    travel_identifier VARCHAR,
    booking_id VARCHAR,
    travel_mode VARCHAR NOT NULL,
    origin VARCHAR NOT NULL,
    transit_points TEXT[] NOT NULL,
    destination VARCHAR NOT NULL,
    travel_intent VARCHAR NOT NULL,
    scheduled_departure_time TIMESTAMP,
    scheduled_arrival_time TIMESTAMP,
    departure_time TIMESTAMP,
    arrival_time TIMESTAMP,
    trip_state VARCHAR(64) NOT NULL,
    travel_group_id UUID NOT NULL
);

INSERT INTO trips VALUES (
    gen_random_uuid(),
    'Air Canada',
    'YYVJKL',
    '11100000',
    'Flight',
    'London',
    '{"Toronto", "Calgary"}',
    'Edmonton',
    'Recreation',
    TIMESTAMP '2004-10-19 10:23:54',
    TIMESTAMP '2004-10-20 10:23:54',
    TIMESTAMP '2004-10-19 10:23:54',
    TIMESTAMP '2004-10-20 11:23:54',
    'complete',
    gen_random_uuid()
);
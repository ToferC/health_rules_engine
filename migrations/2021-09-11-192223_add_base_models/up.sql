-- Your SQL goes here
CREATE TYPE access_level_enum AS ENUM (
    'adminstrator',
    'analyst',
    'employee',
    'research',
    'open')

CREATE TABLE users (
    uid UUID PRIMARY KEY,
    user_instance_uid: UUID NOT NULL,
    email VARCHAR(128) UNIQUE NOT NULL,
    access_level: access_level_enum NOT NULL,
    created_on: TIMESTAMP NOT NULL,
    access_key: VARCHAR(256) NOT NULL,
    approved_by_user_uid: UUID,
)

CREATE TYPE trip_state_enum AS ENUM (
    'planned',
    'in_progress',
    'completed',
    'cancelled',
)

CREATE TABLE trips (
    uid UUID PRIMARY KEY,
    provider VARCHAR(256),
    travel_identifier VARCHAR(256),
    booking_id VARCHAR(256),
    travel_mode VARCHAR(64),
    origin VARCHAR(256),
    transit_points VARCHAR(256)[],
    destination VARCHAR(256),
    travel_intent, VARCHAR(256)
    scheduled_departure_time TIMESTAMP,
    scheduled_arrival_time TIMESTAMP,
    departure_time TIMESTAMP,
    arrival_time TIMESTAMP,
    trip_state trip_state_enum NOT NULL,
)
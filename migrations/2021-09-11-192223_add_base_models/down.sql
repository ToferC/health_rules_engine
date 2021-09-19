-- This file should undo anything in `up.sql`

DROP TYPE IF EXISTS persons;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS trips;
DROP TABLE IF EXISTS places;

DROP TYPE IF EXISTS access_level_enum;
DROP TYPE IF EXISTS trip_state_enum;



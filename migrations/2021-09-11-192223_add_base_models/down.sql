-- This file should undo anything in `up.sql`

DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS trips;

DROP TYPE IF EXISTS access_level_enum;
DROP TYPE IF EXISTS trip_state_enum;


table! {
    trips (id) {
        id -> Uuid,
        trip_provider -> Varchar,
        travel_identifier -> Nullable<Varchar>,
        booking_id -> Nullable<Varchar>,
        travel_mode -> Varchar,
        origin -> Varchar,
        transit_points -> Nullable<Array<Text>>,
        destination -> Varchar,
        travel_intent -> Varchar,
        scheduled_departure_time -> Nullable<Timestamp>,
        scheduled_arrival_time -> Nullable<Timestamp>,
        departure_time -> Nullable<Timestamp>,
        arrival_time -> Nullable<Timestamp>,
        trip_state -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::Access_level_enum;
    users (uid) {
        uid -> Uuid,
        user_instance_uid -> Uuid,
        email -> Varchar,
        access_level -> Access_level_enum,
        created_on -> Timestamp,
        access_key -> Varchar,
        approved_by_user_uid -> Nullable<Uuid>,
    }
}

allow_tables_to_appear_in_same_query!(
    trips,
    users,
);

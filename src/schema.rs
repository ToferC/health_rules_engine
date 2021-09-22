table! {
    countries (id) {
        id -> Uuid,
        country_name -> Varchar,
    }
}

table! {
    persons (id) {
        id -> Uuid,
        birth_date -> Timestamp,
        travel_document_issuer_id -> Uuid,
        approved_access_level -> Varchar,
        approved_access_granularity -> Varchar,
        travel_document_id -> Uuid,
    }
}

table! {
    places (id) {
        id -> Uuid,
        place_name -> Varchar,
        country_id -> Uuid,
    }
}

table! {
    travel_groups (id) {
        id -> Uuid,
    }
}

table! {
    trips (id) {
        id -> Uuid,
        trip_provider -> Varchar,
        travel_identifier -> Nullable<Varchar>,
        booking_id -> Nullable<Varchar>,
        travel_mode -> Varchar,
        origin_place_id -> Uuid,
        transit_point_place_ids -> Array<Uuid>,
        destination_place_id -> Uuid,
        travel_intent -> Varchar,
        scheduled_departure_time -> Nullable<Timestamp>,
        scheduled_arrival_time -> Nullable<Timestamp>,
        departure_time -> Nullable<Timestamp>,
        arrival_time -> Nullable<Timestamp>,
        trip_state -> Varchar,
        travel_group_id -> Uuid,
        person_id -> Uuid,
    }
}

table! {
    use crate::models::Access_level_enum;
    use diesel::sql_types::*;
    users (id) {
        id -> Uuid,
        user_instance_uid -> Uuid,
        email -> Varchar,
        access_level -> Access_level_enum,
        created_on -> Timestamp,
        access_key -> Varchar,
        approved_by_user_uid -> Nullable<Uuid>,
    }
}

allow_tables_to_appear_in_same_query!(
    countries,
    persons,
    places,
    travel_groups,
    trips,
    users,
);

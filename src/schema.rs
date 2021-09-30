table! {
    check_in_results (id) {
        id -> Uuid,
        quarantine_plan_id -> Uuid,
        verifier_id -> Uuid,
        date_time -> Timestamp,
        check_in_complete -> Bool,
    }
}

table! {
    countries (id) {
        id -> Uuid,
        country_name -> Varchar,
        risk_rate -> Float8,
    }
}

table! {
    covid_test (id) {
        id -> Uuid,
        public_health_profile_id -> Uuid,
        test_name -> Varchar,
        test_type -> Varchar,
        date_taken -> Timestamp,
        test_result -> Bool,
    }
}

table! {
    covid_tests (id) {
        id -> Uuid,
        public_health_profile_id -> Uuid,
        test_name -> Varchar,
        test_type -> Varchar,
        date_taken -> Timestamp,
        test_result -> Bool,
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
    postal_addresses (id) {
        id -> Uuid,
        street_address -> Varchar,
        address_locality_id -> Uuid,
        address_region -> Varchar,
        address_country_id -> Uuid,
        postal_code -> Varchar,
        lattitude -> Float8,
        longitude -> Float8,
        additional_info -> Nullable<Text>,
    }
}

table! {
    public_health_profiles (id) {
        id -> Uuid,
        person_id -> Uuid,
        smart_healthcard_pk -> Varchar,
    }
}

table! {
    quarantine_plans (id) {
        id -> Uuid,
        public_health_profile_id -> Uuid,
        date_created -> Timestamp,
        quarantine_required -> Bool,
        confirmation_no_vulnerable -> Bool,
        postal_address_id -> Varchar,
        active -> Bool,
    }
}

table! {
    travel_groups (id) {
        id -> Uuid,
    }
}

table! {
    travel_responses (id) {
        id -> Uuid,
        trip_id -> Uuid,
        response_code -> Varchar,
        random_testing_referral -> Bool,
        date_time -> Timestamp,
        details -> Nullable<Text>,
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
    users (id) {
        id -> Uuid,
        user_instance_uid -> Uuid,
        email -> Varchar,
        access_level -> Varchar,
        created_on -> Timestamp,
        access_key -> Varchar,
        approved_by_user_uid -> Nullable<Uuid>,
    }
}

table! {
    vaccinations (id) {
        id -> Uuid,
        vaccine_id -> Uuid,
        dose_provider -> Varchar,
        location_provided_id -> Uuid,
        provided_on -> Timestamp,
        public_health_profile_id -> Uuid,
    }
}

table! {
    vaccines (id) {
        id -> Uuid,
        vaccine_name -> Varchar,
        manufacturer -> Varchar,
        vaccine_type -> Varchar,
        required_doses -> Int4,
        approved -> Bool,
        approved_on -> Timestamp,
        details -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    check_in_results,
    countries,
    covid_test,
    covid_tests,
    persons,
    places,
    postal_addresses,
    public_health_profiles,
    quarantine_plans,
    travel_groups,
    travel_responses,
    trips,
    users,
    vaccinations,
    vaccines,
);

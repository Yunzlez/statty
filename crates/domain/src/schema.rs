// @generated automatically by Diesel CLI.

diesel::table! {
    charge_sessions (id) {
        id -> Int4,
        date -> Timestamp,
        vehicle_id -> Int4,
        end_soc -> Int4,
        energy -> Float8,
        odometer -> Int4,
    }
}

diesel::table! {
    oauth_clients (id) {
        id -> Int4,
        client_id -> Text,
        client_secret -> Text,
        redirect_uris -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        password -> Text,
    }
}

diesel::table! {
    vehicles (id) {
        id -> Int4,
        name -> Text,
        battery_capacity -> Int4,
        charge_limit -> Float8,
        user_id -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    charge_sessions,
    oauth_clients,
    users,
    vehicles,
);

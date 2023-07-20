// @generated automatically by Diesel CLI.

diesel::table! {
    charge_sessions (id) {
        id -> Int4,
        date -> Timestamp,
        vehicle_id -> Int4,
        end_soc -> Int4,
        energy -> Int4,
        odometer -> Int4,
    }
}

diesel::table! {
    vehicles (id) {
        id -> Int4,
        name -> Text,
        battery_capacity -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    charge_sessions,
    vehicles,
);

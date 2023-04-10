// @generated automatically by Diesel CLI.

diesel::table! {
    logon_history (id) {
        id -> Uuid,
        user -> Int4,
        address -> Varchar,
        started_at -> Timestamptz,
    }
}

diesel::table! {
    logon_sessions (id) {
        id -> Uuid,
        user -> Int4,
        expires_at -> Timestamptz,
        address -> Varchar,
        started_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        role -> Varchar,
        email -> Varchar,
        password -> Varchar,
        updated_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(logon_history -> users (user));
diesel::joinable!(logon_sessions -> users (user));

diesel::allow_tables_to_appear_in_same_query!(
    logon_history,
    logon_sessions,
    users,
);

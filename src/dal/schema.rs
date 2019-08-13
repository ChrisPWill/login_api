table! {
    auth_log (id) {
        id -> Int8,
        email -> Varchar,
        success -> Bool,
        ip_address -> Varchar,
        user_agent -> Varchar,
        date_created -> Timestamptz,
    }
}

table! {
    auth_tokens (id) {
        id -> Int8,
        user_id -> Int8,
        token -> Bytea,
        date_created -> Timestamptz,
        date_expired -> Timestamptz,
        token_type -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int8,
        email -> Varchar,
        password -> Varchar,
        date_created -> Timestamptz,
    }
}

joinable!(auth_tokens -> users (user_id));

allow_tables_to_appear_in_same_query!(auth_log, auth_tokens, users,);

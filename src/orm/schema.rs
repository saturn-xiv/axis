table! {
    agents (id) {
        id -> Integer,
        mac -> Text,
        ip -> Text,
        name -> Text,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

table! {
    logs (id) {
        id -> Integer,
        mac -> Text,
        ip -> Text,
        task -> Text,
        message -> Text,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(agents, logs,);

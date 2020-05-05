table! {
    jobs (id) {
        id -> Integer,
        cid -> Text,
        name -> Text,
        host -> Text,
        reason -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

table! {
    logs (id) {
        id -> Integer,
        host -> Text,
        message -> Text,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(jobs, logs,);

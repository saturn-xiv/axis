table! {
    logs (id) {
        id -> Text,
        uid -> Text,
        host -> Text,
        job -> Text,
        task -> Text,
        result -> Nullable<Text>,
        updated -> Timestamp,
        created -> Timestamp,
    }
}

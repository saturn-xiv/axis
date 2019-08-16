table! {
    agents (id) {
        id -> Integer,
        mac -> Text,
        ip -> Text,
        name -> Text,
        hardware -> Text,
        os -> Text,
        version -> Nullable<Text>,
        online -> Bool,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

table! {
    groups (id) {
        id -> Integer,
        name -> Text,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}

table! {
    groups_agents (id) {
        id -> Integer,
        group_id -> Integer,
        agent_id -> Integer,
        created_at -> Timestamp,
    }
}

table! {
    logs (id) {
        id -> Integer,
        agent_id -> Integer,
        ip -> Text,
        task -> Text,
        message -> Text,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(agents, groups, groups_agents, logs,);

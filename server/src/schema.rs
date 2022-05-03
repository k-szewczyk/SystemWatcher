table! {
    nodes (id) {
        id -> Int4,
        name -> Varchar,
        address -> Varchar,
    }
}

table! {
    stat_types (id) {
        id -> Int4,
        name -> Varchar,
        unit -> Varchar,
    }
}

table! {
    stats (id) {
        id -> Int4,
        node -> Int4,
        stat_type -> Int4,
        value -> Int4,
    }
}

joinable!(stats -> nodes (node));
joinable!(stats -> stat_types (stat_type));

allow_tables_to_appear_in_same_query!(
    nodes,
    stat_types,
    stats,
);

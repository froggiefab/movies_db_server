table! {
    metadata (id) {
        id -> Int4,
        title -> Varchar,
        height -> Int4,
        width -> Int4,
        format -> Varchar,
    }
}

table! {
    movies (id) {
        id -> Int4,
        title -> Varchar,
        synopsis -> Text,
        poster -> Nullable<Varchar>,
        rating -> Nullable<Int4>,
    }
}

allow_tables_to_appear_in_same_query!(
    metadata,
    movies,
);

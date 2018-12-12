table! {
    movies (id) {
        id -> Int4,
        title -> Varchar,
        synopsis -> Text,
        poster -> Nullable<Varchar>,
        rating -> Nullable<Int4>,
    }
}

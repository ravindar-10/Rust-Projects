// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Int4,
        title -> Text,
        author -> Text,
        publication -> Nullable<Text>,
        year -> Nullable<Int4>,
    }
}

// @generated automatically by Diesel CLI.

diesel::table! {
    document (rowid) {
        rowid -> Integer,
        name -> Text,
        modified_date -> Date,
    }
}

diesel::table! {
    tf_idf (word) {
        word -> Text,
        document -> Text,
        tf_idf -> Float,
    }
}

diesel::allow_tables_to_appear_in_same_query!(document, tf_idf,);

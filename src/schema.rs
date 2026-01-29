// @generated automatically by Diesel CLI.

diesel::table! {
    documents (rowid) {
        rowid -> Integer,
        name -> Text,
        modified_date -> Date,
    }
}

diesel::table! {
    word_indexes (rowid) {
        rowid -> Integer,
        word -> Text,
        doc -> Text,
        tf_idf -> Float,
    }
}

diesel::allow_tables_to_appear_in_same_query!(documents, word_indexes,);

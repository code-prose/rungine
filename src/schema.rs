// @generated automatically by Diesel CLI.

diesel::table! {
    documents (rowid) {
        rowid -> Integer,
        name -> Text,
        modified_date -> Date,
    }
}

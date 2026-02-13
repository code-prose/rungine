use crate::schema::{documents, word_indexes};
use chrono::NaiveDateTime;

#[derive(Insertable)]
#[table_name = "documents"]
pub struct NewDocuments<'a> {
    pub name: &'a str,
    pub modified_date: NaiveDateTime,
}

#[derive(Debug, Queryable, AsChangeset)]
#[table_name = "documents"]
pub struct Documents {
    pub rowid: i32,
    pub name: String,
    pub modified_date: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "word_indexes"]
pub struct NewDocumentIndex<'a> {
    pub word: &'a str,
    pub doc: &'a str,
    pub tf_idf: f32,
}

#[derive(Debug, Queryable, AsChangeset)]
#[table_name = "word_indexes"]
pub struct DocumentIndex {
    pub rowid: i32,
    pub word: String,
    pub doc: String,
    pub tf_idf: f32,
}

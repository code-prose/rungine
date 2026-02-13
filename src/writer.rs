use crate::db::establish_connection;
use crate::indexer::Document;
use crate::models::{DocumentIndex, NewDocumentIndex};
use crate::models::{Documents, NewDocuments};
use chrono::{DateTime, NaiveDateTime, Utc};
use std::collections::HashMap;
use std::fs;
use std::time::SystemTime;

type DocPath = String;
type TfIdf = HashMap<String, Vec<(f32, DocPath)>>;

pub struct Writer;

impl Writer {
    pub fn write_index(docs: &Vec<Document>, tf_idfs: &TfIdf) -> std::io::Result<()> {
        use crate::schema::documents::dsl::*;
        use crate::schema::word_indexes::dsl::*;
        use diesel::prelude::*;
        let mut conn = establish_connection();
        for (str, vector) in tf_idfs {
            for i in vector {
                let new_index = NewDocumentIndex {
                    word: &str,
                    doc: &i.1,
                    tf_idf: i.0,
                };
                diesel::insert_into(word_indexes)
                    .values(&new_index)
                    .execute(&mut conn)
                    .expect("Error saving tf-idf mapping");
            }
        }

        let res_index = word_indexes
            .filter(word.ne("placeholder"))
            .load::<DocumentIndex>(&mut conn)
            .expect("Couldn't execute select * from documents");
        println!("{res_index:?}");

        for d in docs.iter() {
            let metadata = fs::metadata(&d.path)?;
            let new_document = NewDocuments {
                name: &d.path,
                modified_date: to_naive_dt(metadata.modified().expect("Expected a NaiveDateTime")),
            };
            diesel::insert_into(documents)
                .values(&new_document)
                .execute(&mut conn)
                .expect("Error saving document");
        }

        let res_docs = documents
            .filter(name.ne("placeholder"))
            .load::<Documents>(&mut conn)
            .expect("Couldn't execute select * from documents");
        println!("{res_docs:?}");

        Ok(())
    }
}

fn to_naive_dt(time: SystemTime) -> NaiveDateTime {
    let utc: DateTime<Utc> = time.into();
    utc.naive_utc()
}

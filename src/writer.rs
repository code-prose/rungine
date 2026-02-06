use crate::db::establish_connection;
use crate::models::{DocumentIndex, NewDocumentIndex};
use crate::models::{Documents, NewDocuments};
use std::collections::HashMap;

type DocPath = String;
type TfIdf = HashMap<String, Vec<(f32, DocPath)>>;
type WordMap = HashMap<String, i64>;

pub struct Writer;

impl Writer {
    pub fn write_index(word_map: &WordMap, tf_idfs: &TfIdf) {
        // use crate::schema::documents::dsl::*;
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
                    .expect("Error saving document");
            }
        }

        let result = word_indexes
            .filter(word.ne("test1"))
            .load::<DocumentIndex>(&mut conn)
            .expect("Couldn't execute select * from documents");

        // TODO: need to write word_map to db

        // println!("Displaying {} documents", result.len());
        // for res in result {
        //     println!("{:?}", res);
        // }
        //
    }
}

fn db_tester() {
    use crate::schema::documents::dsl::*;
    use diesel::prelude::*;
    let mut conn = establish_connection();

    let new_document = NewDocuments {
        name: "test2",
        modified_date: "2026-01-29",
    };

    diesel::insert_into(documents)
        .values(&new_document)
        .execute(&mut conn)
        .expect("Error saving document");

    let result = documents
        .filter(name.ne("test1"))
        .load::<Documents>(&mut conn)
        .expect("Couldn't execute select * from documents");

    println!("Displaying {} documents", result.len());
    for res in result {
        println!("{:?}", res);
    }
}

use diesel::prelude::*;


use crate::models::{DocumentIndex, NewDocumentIndex};
use std::collections::HashMap;
use crate::db::establish_connection;

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
                    tf_idf: i.0
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

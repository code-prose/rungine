#![feature(string_remove_matches)]
#![feature(core_intrinsics)]

use std::collections::HashMap;
use std::io;

#[macro_use]
extern crate diesel;

mod db;
mod models;
mod schema;
mod indexer;
mod parser;
mod writer;

use crate::indexer::Indexer;
use crate::models::{Documents, NewDocuments};
use crate::db::establish_connection;
use crate::parser::Parser;
use crate::writer::Writer;


fn main() -> io::Result<()> {
    let dir = String::from("./tests/docs.gl/gl3/");
    let documents = Parser::iter_dirs(dir)?;


    let word_map = Indexer::create_word_map(&documents);
    let tf_idfs = Indexer::create_tf_idfs(&documents, &word_map);

    Writer::write_index(&word_map, &tf_idfs);

    println!("{:?}", tf_idfs.get("detailC").unwrap());
    Ok(())
}

fn db_tester() {
    use crate::schema::documents::dsl::*;
    use diesel::prelude::*;
    let mut conn = establish_connection();

    let new_document = NewDocuments {
        name: "test2",
        modified_date: "2026-01-29"
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

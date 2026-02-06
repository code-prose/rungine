#![feature(string_remove_matches)]
#![feature(core_intrinsics)]

use std::io;

#[macro_use]
extern crate diesel;

mod db;
mod indexer;
mod models;
mod parser;
mod schema;
mod writer;
mod lexer;

use crate::indexer::Indexer;
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

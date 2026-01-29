#![feature(string_remove_matches)]
#![feature(core_intrinsics)]

use diesel::sql_types::Date;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::intrinsics::log10f32;
use std::io;
use std::path::Path;
use xml::reader::{EventReader, XmlEvent};

#[macro_use]
extern crate diesel;

mod db;
mod schema;
mod models;

struct Document {
    path: DocPath,
    num_words: i64,
    word_freqs: HashMap<String, i64>,
}

type DocPath = String;

struct Indexer;

impl Indexer {
    // this is so so at best... Seems my parsing will need to improve in the future
    fn create_map(
        text: String,
        document_frequency: &mut HashMap<String, i64>,
    ) -> (HashMap<String, i64>, i64) {
        let mut hmap = HashMap::new();
        // this is pretty naive, it's breaking up function calls right now
        let word_iter = text.split_whitespace();
        let num_words = word_iter.clone().count();
        for word in text.split_whitespace() {
            if hmap.contains_key(word) {
                let key_ref = hmap.get_mut(word).unwrap();
                *key_ref += 1;
            } else {
                hmap.insert(word.to_string(), 1);

                if document_frequency.contains_key(word) {
                    let key_ref = document_frequency.get_mut(word).unwrap();
                    *key_ref += 1;
                } else {
                    document_frequency.insert(word.to_string(), 1);
                }
            }
        }
        (hmap, num_words as i64)
    }

    fn calc_tf_idf(num_docs: f32, num_docs_appear: f32, term_count: f32, word_count: f32) -> f32 {
        Indexer::calc_idf(num_docs, num_docs_appear) * Indexer::calc_tf(term_count, word_count)
    }

    fn calc_idf(num_docs: f32, num_docs_appear: f32) -> f32 {
        log10f32(num_docs / num_docs_appear)
    }

    fn calc_tf(term_count: f32, word_count: f32) -> f32 {
        term_count / word_count
    }
}

#[derive(Debug)]
enum ParserError {
    XmlParserError { err: xml::reader::Error },
    HtmlParserError,
    XHtmlParserError { err: xml::reader::Error },
    PdfParserError,
    FileTypeError,
}

// Start with support xml then we can move onto HTML and PDF
impl Parser {
    fn parse(file: std::fs::File, fp: &str) -> Result<String, ParserError> {
        let idx = fp.rfind('.').unwrap();
        let file_ext = fp.split_at(idx).1;

        match file_ext {
            ".xhtml" => Parser::parse_xhtml(file),
            ".html" => Parser::parse_html(file),
            ".pdf" => Parser::parse_pdf(file),
            ".xml" => Parser::parse_xml(file),
            _ => Err(ParserError::FileTypeError),
        }
    }

    fn parse_xml(file: std::fs::File) -> Result<String, ParserError> {
        todo!()
    }

    fn parse_xhtml(file: std::fs::File) -> Result<String, ParserError> {
        let mut doc = String::from("");
        let er = EventReader::new(file);
        for event in er.into_iter() {
            let event = event.map_err(|err| ParserError::XHtmlParserError { err: err })?;
            if let XmlEvent::Characters(text) = event {
                doc.push_str(&text);
            }
        }

        Ok(Self::clean_text(doc))
    }

    fn clean_text(mut text: String) -> String {
        text.remove_matches("\n");
        let single_space = Regex::new(r"\s+").unwrap().replace_all(&text, " ");
        single_space.to_string()
    }

    fn parse_html(file: std::fs::File) -> Result<String, ParserError> {
        todo!()
    }

    fn parse_pdf(file: std::fs::File) -> Result<String, ParserError> {
        todo!()
    }
}

fn open_file<P: AsRef<Path>>(file_name: P) -> Result<std::fs::File, std::io::Error> {
    match File::open(file_name) {
        Ok(contents) => Ok(contents),
        Err(e) => Err(e),
    }
}

struct Parser;

fn holder_while_refactor() -> io::Result<()> {
    let dir = String::from("./tests/docs.gl/gl3/");
    let mut documents = Vec::new();
    let mut docs_with_word = HashMap::new();
    for fp in std::fs::read_dir(&dir)? {
        let fp = fp?.path();
        let file = match open_file(&fp) {
            Ok(f) => f,
            Err(err) => {
                eprintln!("Failed to read content. Shutting down:\nError:\n{err}");
                std::process::exit(1)
            }
        };
        println!("{fp:?}");
        let doc = Parser::parse(file, fp.clone().into_os_string().to_str().unwrap()).unwrap();
        let (hmap, num_words) = Indexer::create_map(doc, &mut docs_with_word);

        documents.push(Document {
            path: fp.to_str().unwrap().to_string(),
            num_words: num_words,
            word_freqs: hmap,
        });
    }
    dbg!(&docs_with_word);

    let mut word_to_doc: HashMap<String, i64> = HashMap::new();
    for doc in documents.iter() {
        for word in doc.word_freqs.keys() {
            if word_to_doc.contains_key(word) {
                let key_ref = word_to_doc.get_mut(word).unwrap();
                *key_ref += 1;
            } else {
                word_to_doc.insert(word.clone().to_string(), 1);
            }
        }
    }

    let mut tf_idfs: HashMap<String, Vec<(f32, DocPath)>> = HashMap::new();
    for doc in documents.iter() {
        for word in doc.word_freqs.keys() {
            if tf_idfs.contains_key(word) {
                let key_ref = tf_idfs.get_mut(word).unwrap();
                key_ref.push((
                    Indexer::calc_tf_idf(
                        documents.len() as f32,
                        word_to_doc.get(word).unwrap().clone() as f32,
                        doc.word_freqs.get(word).unwrap().clone() as f32,
                        doc.num_words.clone() as f32,
                    ),
                    doc.path.clone(),
                ));
            } else {
                let mut t_vec: Vec<(f32, DocPath)> = Vec::new();
                t_vec.push((
                    Indexer::calc_tf_idf(
                        documents.len() as f32,
                        word_to_doc.get(word).unwrap().clone() as f32,
                        doc.word_freqs.get(word).unwrap().clone() as f32,
                        doc.num_words.clone() as f32,
                    ),
                    doc.path.clone(),
                ));

                tf_idfs.insert(word.to_string(), t_vec);
            }
        }
    }
    println!("{:?}", tf_idfs.get("detailC").unwrap());

    Ok(())
}

fn main() {
    println!("Hello, world!");
}

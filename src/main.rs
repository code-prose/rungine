#![feature(string_remove_matches)]

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::path::Path;
use xml::reader::{EventReader, XmlEvent};

fn main() -> io::Result<()> {
    let dir = String::from("./tests/docs.gl/gl3/");
    let mut documents = Vec::new();
    for fp in std::fs::read_dir(&dir)? {
        let fp = fp?.path();
        let file = match open_file(&fp) {
            Ok(f) => f,
            Err(err) => {
                eprintln!("Failed to read content. Shutting down:\nError:\n{err}");
                std::process::exit(1)
            }
        };
        let doc = Parser::parse(file, fp.clone().into_os_string().to_str().unwrap()).unwrap();
        let (hmap, num_words) = Indexer::create_map(doc);
        println!("{hmap:?}");
        
        let tfidf = Indexer::create_index(hmap, num_words);
        documents.push(Document{path: fp.to_str().unwrap().to_string(), tfidfs: tfidf});

    }

    Ok(())
}


struct TFIDF {
    term: String,
    tf:  f32,
    idf: f32
}

struct Document {
    path: String,
    tfidfs: Vec<TFIDF>
}

struct Indexer;

impl Indexer {
    fn create_map(text: String) -> (HashMap<String, i64>, i64) {
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
            }
        }
        (hmap, num_words as i64)
    }

    fn create_index(hmap: HashMap<String, i64>, count: i64) -> Vec<TFIDF> {
        let mut tfidfs: Vec<TFIDF> = Vec::new();
        for (k, v) in hmap.into_iter() {
           tfidfs.push(TFIDF{
                term: k,
                tf: v as f32,
                idf: (v as f32 / count as f32)
           });
        }
        tfidfs
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

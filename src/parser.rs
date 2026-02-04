use xml::reader::{EventReader, XmlEvent};
use regex::Regex;
use std::fs::File;
use std::path::Path;

use lopdf::Document as Pdf;

use crate::indexer::{Document, Indexer};
use std::collections::HashMap;


impl AsRef<Path> for std::fs::File {
    fn as_ref(&self) -> &Path {
        Ok(())
    }
}

#[derive(Debug)]
enum ParserError {
    HtmlParserError,
    XmlParserError { err: xml::reader::Error },
    PdfParserError,
    FileTypeError,
}

pub struct Parser;


impl Parser {
    fn parse<P: AsRef<Path> + std::io::Read>(file: P, fp: &str) -> Result<String, ParserError> {
        let idx = fp.rfind('.').unwrap();
        let file_ext = fp.split_at(idx).1;

        match file_ext {
            ".xhtml" => Parser::parse_xml(file),
            ".xml" => Parser::parse_xml(file),
            ".html" => Parser::parse_html(file),
            ".pdf" => Parser::parse_pdf(file),
            _ => Err(ParserError::FileTypeError),
        }
    }

    fn parse_xml<P: AsRef<Path> + std::io::Read>(file: P) -> Result<String, ParserError> {
        let mut doc = String::from("");
        let er = EventReader::new(file);
        for event in er.into_iter() {
            let event = event.map_err(|err| ParserError::XmlParserError { err: err })?;
            if let XmlEvent::Characters(text) = event {
                doc.push_str(&text);
            }
        }

        Ok(Self::clean_text(doc))
    }

    pub fn iter_dirs(dir_path: String) -> Result<Vec<Document>, std::io::Error> {
        let mut documents = Vec::new();
        let mut docs_with_word = HashMap::new();
        for fp in std::fs::read_dir(&dir_path)? {
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
        Ok(documents)

    }

    fn clean_text(mut text: String) -> String {
        text.remove_matches("\n");
        let single_space = Regex::new(r"\s+").unwrap().replace_all(&text, " ");
        single_space.to_string()
    }

    fn parse_html<P: AsRef<Path>>(file: P) -> Result<String, ParserError> {
        todo!()
    }

    fn parse_pdf<P: AsRef<Path>>(file: P) -> Result<String, ParserError> {
        let mut doc = String::from("");
        match Pdf::load(file) {
            Ok(document) => {
                let pages = document.get_pages();
     
                for (i, _) in pages.iter().enumerate() {
                    let page_number = (i + 1) as u32;
                    let text = document.extract_text(&[page_number]);
                    doc.push_str(&text.unwrap_or_default());
                }
                // could return page # for pdfs...
     
                println!("Text:\n{doc}");
            }
            Err(err) => eprintln!("Error: {}", err),
        }
        todo!()
    }
}

fn open_file<P: AsRef<Path>>(file_name: P) -> Result<std::fs::File, std::io::Error> {
    match File::open(file_name) {
        Ok(contents) => Ok(contents),
        Err(e) => Err(e),
    }
}

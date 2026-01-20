use std::fs::File;
use std::collections::HashMap;
use xml::reader::{XmlEvent, EventReader};
use std::io;




fn main() -> io::Result<()> {
    let dir = String::from("./tests/docs.gl/gl3/").to_string();
    for fp in std::fs::read_dir(&dir)? {
        let fp = fp.unwrap().file_name();
        let full_path = dir.clone() + fp.to_str().unwrap();
        let file = match open_file(&full_path) {
            Ok(contents) => contents,
            Err(err) => {
                eprintln!("Failed to read content. Shutting down:\nError:\n{err}");
                std::process::exit(1)
            }
        };
        let doc = Parser::parse(file, &full_path);
        println!("{doc:?}");
    }
    

    Ok(())
}


#[derive(Debug)]
enum ParserError {
    XmlParserError {
        err: xml::reader::Error
    },
    HtmlParserError,
    XHtmlParserError {
        err: xml::reader::Error
    },
    PdfParserError,
    FileTypeError
}

// Start with support xml then we can move onto HTML and PDF 
impl Parser {
    fn parse(file: std::fs::File, fp: &str) -> Result<String, ParserError> {
        let idx = fp.rfind('.').unwrap();
        let file_ext = fp.split_at(idx).1;
        println!("{:?}", file_ext);

        match file_ext {
            ".xhtml" => Parser::parse_xhtml(file),
            ".html" => Parser::parse_html(file),
            ".pdf" => Parser::parse_pdf(file),
            ".xml" => Parser::parse_xml(file),
            _ => {
                Err(ParserError::FileTypeError)
            }
        }
    }

    fn parse_xml(file: std::fs::File) -> Result<String, ParserError> {
        todo!()
    }

    fn parse_xhtml(file: std::fs::File) -> Result<String, ParserError> {
        let mut doc = String::from("");
        let er = EventReader::new(file);
        for event in er.into_iter() {
            // println!("{event:?}");
            let event = event.map_err({
                |err| ParserError::XHtmlParserError{
                    err: err
                }
            })?;
            if let XmlEvent::Characters(text) = event {
                doc.push_str(&text);
            }
        }

        Ok(doc)
    }

    fn parse_html(file: std::fs::File) -> Result<String, ParserError> {
        todo!()
    }

    fn parse_pdf(file: std::fs::File) -> Result<String, ParserError> {
        todo!()
    }
}

fn open_file(file_name: &str) -> Result<std::fs::File, std::io::Error> {
    match File::open(file_name) {
        Ok(contents) => Ok(contents),
        Err(e) => Err(e)
    }
}

struct Parser;

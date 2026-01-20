use std::fs::File;
use std::collections::HashMap;
use xml::EventReader;




fn main() {

    let fp = String::from("./tests/docs.gl/gl3/glActiveTexture.xhtml").to_string();

    let file = match open_file(&fp) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Failed to read content. Shutting down:\nError:\n{err}");
            std::process::exit(1)
        }

    };
    Parser::parse(file, &fp);
}


// Start with support xml then we can move onto HTML and PDF 
impl Parser {
    fn parse(file: std::fs::File, fp: &str) -> Vec<String> {
        let idx = fp.rfind('.').unwrap();
        let file_ext = fp.clone().split_at(idx).1;

        match file_ext {
            "xhtml" => Parser::parse_xhtml(file),
            "html" => Parser::parse_html(file),
            "pdf" => Parser::parse_pdf(file),
            "xml" => Parser::parse_xml(file),
            _ => {
                eprintln!("File-type not supported");
                std::process::exit(1)
            }
        }
    }
    fn parse_xml(file: std::fs::File) -> Vec<String> {
        EventReader::new(file);
    }

    fn parse_xhtml(file: std::fs::File) -> Vec<String> {
        todo!()
    }

    fn parse_html(file: std::fs::File) -> Vec<String> {
        todo!()
    }

    fn parse_pdf(file: std::fs::File) -> Vec<String> {
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

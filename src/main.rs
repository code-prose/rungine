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
    EventReader::new(file);
}

struct Parser;

// Start with support xml then we can move onto HTML and PDF 
impl Parser {
    fn parse_xml() -> Vec<String> {
        todo!()
    }

    fn parse_html() -> Vec<String> {
        todo!()
    }

    fn parse_pdf() -> Vec<String> {
        todo!()
    }
}

fn open_file(file_name: &str) -> Result<std::fs::File, std::io::Error> {
    match File::open(file_name) {
        Ok(contents) => Ok(contents),
        Err(e) => Err(e)
    }
}

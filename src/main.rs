use std::fs::read_to_string;
use std::collections::HashMap;




fn main() {

    let fp = String::from("./tests/std-fs-file.html2").to_string();
    let file = match read_file(&fp) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Failed to read content. Shutting down:\nError:\n{err}");
            std::process::exit(1)
        }

    };
    println!("{}", file);
}

struct Parser;

// let's do a naive strip for now and then we can come back and then of a better approach
impl Parser {
    fn parse_xml() -> Vec<String> {
        todo!()
    }
    fn parse_html() -> Vec<String> {
        todo!()
    }
}

fn read_file(file_name: &str) -> Result<String, std::io::Error> {
    match read_to_string(file_name) {
        Ok(contents) => Ok(contents),
        Err(e) => Err(e)
    }
}

use std::fs::read_to_string;
use std::collections::HashMap;
use scraper::Selector;
use scraper::Html;




fn main() {

    let file = read_to_string("./tests/std-fs-file.html").unwrap();
    println!("{}", file);
    let document = Html::parse_document(file.as_str());
    // println!("{:?}", document);
    let paragraph = Selector::parse("p").unwrap();
    // println!("{:?}", paragraph);
    for element in document.select(&paragraph) {
        println!("{:?}", element.value().attr("value"));
    }
}

struct Parser;

impl Parser {
    fn strip_tags(element: &str) -> Vec<String> {
        element.matches


    }

}

// impl parse for Parser {
//     fn parse(file: String) -> HashMap<String, u16> {
//         todo!()
//     }
// }

fn html_parser() {
    todo!()
}

fn iter_dirs() {
    todo!()
}

fn read_file(file_name: String) -> Vec<String> {
    todo!()
}

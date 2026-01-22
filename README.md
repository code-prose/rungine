## Rungine
TF-IDF implemented in Rust!

## References
- [TF-IDF](https://en.wikipedia.org/wiki/Tf%E2%80%93idf)
- [XML-RS](https://crates.io/crates/xml-rs)
- [Rust Docs](https://rust-lang.org/tools/install/)
- [HTML Parser]()

### Information
- [XML-RS](https://crates.io/crates/xml-rs) is a [SAX](https://en.wikipedia.org/wiki/Simple_API_for_XML) parser

### Left to implement
- [x] Parse directories
- [x] Parse files
- [x] Create index
- [ ] Write to DB? (NoSQL?)
- [ ] Better "word" splitting [create_map](./src/main.rs:52)
- [ ] Multithread?
- [ ] PDF, HTML support

### Questions
- Do I want to grab hrefs to further expand on the accuracy? i.e. using number of links to as an indicator
- Do I need to re-index old documents if I add new ones?

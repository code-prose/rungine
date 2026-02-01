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
- [x] Create tf-idfs
- [x] Creating word -> sorted tf-idf mapping (Index)
- [x] Write to DB? (NoSQL?)
- [ ] Better "word" splitting [create_map](./src/main.rs:52)
- [ ] Multithread?
- [ ] PDF, HTML support

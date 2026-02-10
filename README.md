## Rungine
TF-IDF implemented in Rust!

## References
- [TF-IDF](https://en.wikipedia.org/wiki/Tf%E2%80%93idf)
- [Lexer](https://en.wikipedia.org/wiki/Lexical_analysis)
- [XML-RS](https://crates.io/crates/xml-rs)
- [HTML Parser]()
- [PDF Parser](https://github.com/jrmuizel/pdf-extract)

### Information
- [XML-RS](https://crates.io/crates/xml-rs) is a [SAX](https://en.wikipedia.org/wiki/Simple_API_for_XML) parser

### Left to implement
- [x] Parse directories
- [x] Parse files
- [x] Create tf-idfs
- [x] Creating word -> sorted tf-idf mapping (Index)
- [x] Write to DB? (NoSQL?)
- [x] Better "word" splitting [create_map](./src/main.rs:52) (Outdated link) - Solution: [Lexer](./src/lexer.rs)
- [ ] Multithread?
- [x] PDF Support
- [ ] HTML support
- [ ] Testing using idiomatic rust instead of functional
- [ ] Clean up non-idomatic rust - i.e. `to_str().clone()....bullshit`

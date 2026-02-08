#[derive(Debug)]
pub struct Lexer<'a> {
    content: &'a [char],
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a [char]) -> Self {
        Self { content }
    }

    fn trim_left(&mut self) {
        while self.content.len() > 0 && (self.content[0].is_whitespace()) {
            self.content = &self.content[1..];
        }
    }

    fn next_token(&mut self) -> Option<&'a [char]> {
        // trimming whitespace
        self.trim_left();
        if self.content.is_empty() {
            return None;
        }

        if self.content[0].is_numeric() {
            while self.content.len() > 0 && self.content[0].is_alphanumeric() {
                let mut len = 0;
                while len < self.content.len() && self.content[len].is_alphanumeric() {
                    len += 1;
                }
                let tok = &self.content[0..len];
                self.content = &self.content[len..];
                Some(tok);
            }
        }
        
        if self.content[0].is_alphabetic() {
            while self.content.len() > 0 && self.content[0].is_alphanumeric() {
                let mut len = 0;
                while len < self.content.len() && self.content[len].is_alphanumeric() {
                    len += 1;
                }
                let tok = &self.content[0..len];
                self.content = &self.content[len..];
                Some(tok);
            }
        }

        todo!()
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = &'a [char];
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

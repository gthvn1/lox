use crate::token::{Token, TokenType};

pub struct Scanner<'a> {
    source: &'a [u8],
    start: usize,   // points to the first character in lexeme being scanned
    current: usize, // point at the character currently being considered
    line: usize,    // track source line
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a Vec<u8>) -> Scanner<'a> {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token<'a>> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        tokens.push(Token::new(TokenType::Eof, "", None, self.line));
        tokens
    }

    fn scan_token(&mut self) {
        _ = self;
        todo!("scan the token");
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

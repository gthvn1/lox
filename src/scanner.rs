use crate::token::{Token, TokenType};

fn report_error(line: usize, msg: &str) -> bool {
    println!("[line {line}] Error: {msg}");
    true
}

pub struct Scanner<'a> {
    source: &'a [u8],
    tokens: Vec<Token<'a>>,

    start: usize,   // points to the first character in lexeme being scanned
    current: usize, // point at the character currently being considered
    line: usize,    // track source line

    had_error: bool,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a [u8]) -> Scanner<'a> {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            had_error: false,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token<'a>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        // Move ownership out and add Eof token before returning
        let mut tokens = std::mem::take(&mut self.tokens);
        tokens.push(Token::new(TokenType::Eof, "", None, self.line));
        tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            b'(' => self.add_token(TokenType::LeftParen),
            b')' => self.add_token(TokenType::RightParen),
            b'{' => self.add_token(TokenType::LeftBrace),
            b'}' => self.add_token(TokenType::RightBrace),
            b',' => self.add_token(TokenType::Comma),
            b'.' => self.add_token(TokenType::Dot),
            b'-' => self.add_token(TokenType::Minus),
            b'+' => self.add_token(TokenType::Plus),
            b';' => self.add_token(TokenType::Semicolon),
            b'*' => self.add_token(TokenType::Star),
            _ => self.had_error = report_error(self.line, "Unexpected character"),
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = std::str::from_utf8(&self.source[self.start..self.current])
            .expect("lexeme should be valid UTF-8");

        self.tokens
            .push(Token::new(token_type, lexeme, None, self.line));
    }

    // returns the current byte and update the current position
    fn advance(&mut self) -> u8 {
        let b = self.source[self.current];
        self.current += 1;
        b
    }

    // returns true if we are at then end of the source
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

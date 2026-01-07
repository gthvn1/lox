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
            b'!' => {
                if self.match_current(b'=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            b'=' => {
                if self.match_current(b'=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            b'<' => {
                if self.match_current(b'=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            b'>' => {
                if self.match_current(b'=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            b'/' => {
                if self.match_current(b'/') {
                    while (self.peek() != b'\n') && !self.is_at_end() {
                        _ = self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            b' ' | b'\r' | b'\t' => {}
            b'\n' => self.line += 1,
            b'"' => self.string(),
            _ => self.had_error = report_error(self.line, "Unexpected character"),
        }
    }

    // read string literal
    fn string(&mut self) {
        // We try to match the next "
        while (self.peek() != b'"') && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.had_error = report_error(self.line, "Unterminated string.");
            return;
        }

        // We are at the closing ", consume it
        self.advance();

        // Trim the surrounding quotes.
        let value = std::str::from_utf8(&self.source[self.start + 1..self.current - 1])
            .unwrap()
            .to_owned();
        self.add_token_and_literal(TokenType::String, Some(value));
    }

    // peek return the current character  without consuming the
    fn peek(&self) -> u8 {
        if self.is_at_end() {
            b'\0'
        } else {
            self.source[self.current]
        }
    }

    // return true if the current character is the expected one. And if it
    // is it consume it.
    fn match_current(&mut self, expected: u8) -> bool {
        if self.is_at_end() {
            false
        } else if self.source[self.current] != expected {
            false
        } else {
            self.current += 1;
            true
        }
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

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_and_literal(token_type, None);
    }

    fn add_token_and_literal(&mut self, token_type: TokenType, literal: Option<String>) {
        let lexeme = std::str::from_utf8(&self.source[self.start..self.current])
            .expect("lexeme should be valid UTF-8");

        self.tokens
            .push(Token::new(token_type, lexeme, literal, self.line));
    }
}

fn is_digit(c: u8) -> bool {
    b'0' <= c && c <= b'9'
}

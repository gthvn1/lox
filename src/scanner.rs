use std::fmt;

pub struct Scanner<'a> {
    source: &'a Vec<u8>,
}
pub struct Token {}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        _ = self;
        write!(f, "todo")
    }
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a Vec<u8>) -> Scanner<'a> {
        Scanner { source }
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        _ = self.source;
        Vec::new()
    }
}

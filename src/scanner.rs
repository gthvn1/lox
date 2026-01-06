use crate::token::Token;

pub struct Scanner<'a> {
    source: &'a Vec<u8>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a Vec<u8>) -> Scanner<'a> {
        Scanner { source }
    }

    pub fn scan_tokens(&self) -> Vec<Token<'a>> {
        _ = self.source;
        Vec::new()
    }
}

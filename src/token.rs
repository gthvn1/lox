use std::fmt;

pub enum Literal {
    String(String),
    Number(f64),
    Null,
}

pub struct Token<'a> {
    token_type: TokenType,
    literal: Literal, // It is the runtime object that will be used by interpreter
    lexeme: &'a str,  // The raw substring in the source
    line: usize,
}

#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

impl<'a> Token<'a> {
    pub fn new(ty: TokenType, lexeme: &'a str, literal: Literal, line: usize) -> Token<'a> {
        Token {
            token_type: ty,
            literal,
            lexeme,
            line,
        }
    }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        _ = self.line; // allow to remove the not use warning when building
        let literal = match &self.literal {
            Literal::Null => "null".to_string(),
            Literal::String(s) => s.clone(),
            Literal::Number(x) => x.to_string(),
        };

        write!(f, "{:?} {} {}", self.token_type, self.lexeme, literal)
    }
}

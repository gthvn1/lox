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

impl TokenType {
    pub fn of_string(ident: &str) -> Option<TokenType> {
        match ident {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "fun" => Some(TokenType::Fun),
            "for" => Some(TokenType::For),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            _ => None,
        }
    }
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

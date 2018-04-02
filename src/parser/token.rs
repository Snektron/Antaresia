use ast::{Literal, DataType};

#[derive(Clone, Debug, PartialEq)]
pub enum Keyword {
    If,
    Else,
    While,
    Struct,
    Func,
    Return,
    As,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Whitespace,
    Comment(String),
    
    Ident(String),
    Literal(Literal),
    DataType(DataType),
    Keyword(Keyword),

    BraceOpen,
    BraceClose,
    BracketOpen,
    BracketClose,
    ParenOpen,
    ParenClose,

    Arrow,
    Comma,
    Dot,
    Equals,
    EqualsEquals,
    Exclamation,
    ExclamationEquals,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Semicolon,
    Tilde,
    Left,
    Right,
    LeftEq,
    RightEq,
    LeftLeft,
    RightRight,
    Hat,
    Ampersand,
    Pipe,
}
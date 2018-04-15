use std::fmt;

#[derive(Clone)]
pub enum Token {
    Comma,
    Semi,
    BraceOpen,
    BraceClose,
    ParenOpen,
    ParenClose,
    BracketOpen,
    BracketClose,
    Equals,
    Colon,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Tilde,
    Exclamation,
    Arrow,
    
    If,
    Else,
    While,
    Return,
    Func,
    Struct,
    As,

    U8,
    Void,

    Integer(String),
    Ident(String),

    Comment,
    Whitespace
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::Comma => write!(f, ","),
            Token::Semi => write!(f, ";"),
            Token::BraceOpen => write!(f, "{{"),
            Token::BraceClose => write!(f, "}}"),
            Token::ParenOpen => write!(f, "("),
            Token::ParenClose => write!(f, ")"),
            Token::BracketOpen => write!(f, "["),
            Token::BracketClose => write!(f, "]"),
            Token::Equals => write!(f, "="),
            Token::Colon => write!(f, ":"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Percent => write!(f, "%"),
            Token::Tilde => write!(f, "~"),
            Token::Exclamation => write!(f, "!"),
            Token::Arrow => write!(f, "->"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::While => write!(f, "while"),
            Token::Return => write!(f, "return"),
            Token::Func => write!(f, "func"),
            Token::Struct => write!(f, "struct"),
            Token::As => write!(f, "as"),
            Token::U8 => write!(f, "u8"),
            Token::Void => write!(f, "void"),
            Token::Integer(ref s) => write!(f, "{}", s),
            Token::Ident(ref s) => write!(f, "{}", s),
            Token::Comment => write!(f, "Comment"),
            Token::Whitespace => write!(f, "Whitespace"),
        }
    }
}
use std::io::{self, Read, Bytes};
use std::iter::{Iterator, Peekable};
use std::fmt;
use std::error;
use phf::Map;
use phf_builder::Map as MapBuilder;
use parser::token::{Token, Keyword};
use ast::{Literal, DataType};

lazy_static! {
    static ref KEYWORDS: Map<&'static str, Keyword> = {
        let mut m = MapBuilder::new();
        m.entry("if", Keyword::If);
        m.entry("else", Keyword::Else);
        m.entry("while", Keyword::While);
        m.entry("struct", Keyword::Struct);
        m.entry("func", Keyword::Func);
        m.entry("return", Keyword::Return);
        m.entry("as", Keyword::As);
        m.build()
    };

    static ref DATATYPES: Map<&'static str, DataType> = {
        let mut m = MapBuilder::new();
        m.entry("u8", DataType::U8);
        m.entry("void", DataType::Void);
        m.build()
    };
}

pub struct Lexer<R>
where R: Read {
    input: Peekable<Chars<R>>
}

impl<R> Lexer<R>
where R: Read {
    pub fn new(input: R) -> Lexer<R> {
        Lexer {
            input: Chars::new(input).peekable()
        }
    }

    fn peek(&mut self) -> Result<Option<char>, Error> {
        match self.input.peek() {
            Some(&Ok(c)) => Ok(Some(c)),
            Some(&Err(_)) => Err(Error::Other(self.input.next().unwrap().unwrap_err())),
            None => Ok(None)
        }
    }

    fn consume(&mut self) -> Result<Option<char>, Error> {
        match self.input.next() {
            Some(Ok(c)) => Ok(Some(c)),
            Some(Err(e)) => Err(Error::Other(e)),
            None => Ok(None)
        }
    }

    fn consume_while<'a, P>(&'a mut self, predicate: P) -> ConsumeWhile<'a, R, P>
    where R: 'a,
          P: FnMut(char) -> bool {
        ConsumeWhile {
            lexer: self,
            predicate,
            done: false
        }
    }

    fn next_inner(&mut self, c: char) -> Result<Token, Error> {
        match c {
            '{' => self.consume().and(Ok(Token::BraceOpen)),
            '}' => self.consume().and(Ok(Token::BraceClose)),
            '[' => self.consume().and(Ok(Token::BracketOpen)),
            ']' => self.consume().and(Ok(Token::BracketClose)),
            '(' => self.consume().and(Ok(Token::ParenOpen)),
            ')' => self.consume().and(Ok(Token::ParenClose)),
            ',' => self.consume().and(Ok(Token::Comma)),
            '.' => self.consume().and(Ok(Token::Dot)),
            '+' => self.consume().and(Ok(Token::Plus)),
            '*' => self.consume().and(Ok(Token::Star)),
            '%' => self.consume().and(Ok(Token::Percent)),
            ';' => self.consume().and(Ok(Token::Semicolon)),
            '^' => self.consume().and(Ok(Token::Hat)),
            '|' => self.consume().and(Ok(Token::Pipe)),
            '&' => self.consume().and(Ok(Token::Ampersand)),
            '~' => self.consume().and(Ok(Token::Tilde)),
            '-' => self.consume().and(self.peek()).and_then(|c| {
                match c {
                    Some('>') => self.consume().and(Ok(Token::Arrow)),
                    _ => Ok(Token::Minus)
                }
            }),
            '=' => self.consume().and(self.peek()).and_then(|c| {
                match c {
                    Some('=') => self.consume().and(Ok(Token::EqualsEquals)),
                    _ => Ok(Token::Equals)
                }
            }),
            '!' => self.consume().and(self.peek()).and_then(|c| {
                match c {
                    Some('=') => self.consume().and(Ok(Token::ExclamationEquals)),
                    _ => Ok(Token::Exclamation)
                }
            }),
            '<' => self.consume().and(self.peek()).and_then(|c| {
                match c {
                    Some('<') => self.consume().and(Ok(Token::LeftLeft)),
                    Some('=') => self.consume().and(Ok(Token::LeftEq)),
                    _ => Ok(Token::Left)
                }
            }),
            '>' => self.consume().and(self.peek()).and_then(|c| {
                match c {
                    Some('>') => self.consume().and(Ok(Token::RightRight)),
                    Some('=') => self.consume().and(Ok(Token::RightEq)),
                    _ => Ok(Token::Right)
                }
            }),
            '/' => self.consume().and(self.peek()).and_then(|c| {
                match c {
                    Some('/') => {
                        self.consume().and(
                            self.consume_while(|c| c != '\n')
                                .collect::<Result<String, _>>()
                                .map(|comment| Token::Comment(comment))
                        )
                    }
                    _ => Ok(Token::Slash)
                }
            }),
            c if c.is_alphabetic() => {
                self.consume_while(|c| c.is_alphanumeric())
                    .collect::<Result<String, _>>()
                    .map(|s|
                        KEYWORDS
                        .get(s.as_str())
                        .map(|kw| Token::Keyword(kw.clone()))
                        .or_else(||
                            DATATYPES
                            .get(s.as_str())
                            .map(|dt| Token::DataType(dt.clone()))
                        )
                        .unwrap_or_else(|| Token::Ident(s))
                    )
            },
            c if c.is_whitespace() => {
                self.consume_while(|c| c.is_whitespace())
                    .find(|e| e.is_err())
                    .map(|e| Err(e.unwrap_err()))
                    .unwrap_or(Ok(Token::Whitespace))
            },
            c if c.is_digit(10) => {
                self.consume_while(|c| c.is_digit(10))
                    .collect::<Result<String, _>>()
                    .map(|s| Token::Literal(Literal::Integer(s.parse::<i64>().unwrap())))
            },
            _ => Err(Error::UnexpectedChar(c))
        }
    }
}

impl<R> Iterator for Lexer<R>
where R: Read {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.peek() {
            Ok(None) => None,
            Ok(Some(c)) => Some(self.next_inner(c)),
            Err(e) => Some(Err(e))
        }
    }
}

struct Chars<R>
where R: Read {
    input: Bytes<R>
}

impl<R> Chars<R>
where R: Read {
    fn new(input: R) -> Chars<R> {
        Chars {
            input: input.bytes()
        }
    }
}

impl<R> Iterator for Chars<R>
where R: Read {
    type Item = Result<char, io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.input.next().map(|r| {
            r.map(|c| c as char)
        })
    }
}

#[derive(Debug)]
pub enum Error {
    UnexpectedChar(char),
    Other(io::Error)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::UnexpectedChar(ref c) => write!(f, "Unexpected character {}", c),
            Error::Other(ref err) => err.fmt(f)
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::UnexpectedChar(_) => "Unexpected character",
            Error::Other(_) => "Other"
        }
    }
}

struct ConsumeWhile<'a, R, P>
where R: Read + 'a,
      P: FnMut(char) -> bool {
    lexer: &'a mut Lexer<R>,
    predicate: P,
    done: bool
}

impl<'a, R, P> Iterator for ConsumeWhile<'a, R, P>
where R: Read + 'a,
      P: FnMut(char) -> bool {
    type Item = Result<char, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            match self.lexer.peek() {
                Ok(Some(c)) => {
                    if (self.predicate)(c) {
                        Some(Ok(self.lexer.consume().unwrap().unwrap()))
                    } else {
                        self.done = true;
                        None
                    }
                },
                Ok(None) => {
                    self.done = true;
                    None
                }
                Err(e) => Some(Err(e))
            }
        }
    }
}

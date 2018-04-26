use std::iter::{Iterator, Peekable};
use std::str::CharIndices;
use std::error::Error;
use std::default::Default;
use std::fmt;
use phf::Map;
use phf_builder::Map as MapBuilder;
use parser::token::Token;
use parser::span::Location;

lazy_static! {
    static ref KEYWORDS: Map<&'static str, Token> = {
        let mut m = MapBuilder::new();
        m.entry("if", Token::If);
        m.entry("else", Token::Else);
        m.entry("while", Token::While);
        m.entry("func", Token::Func);
        m.entry("return", Token::Return);
        m.entry("struct", Token::Struct);
        m.entry("as", Token::As);
        m.entry("u8", Token::U8);
        m.entry("void", Token::Void);
        m.build()
    };
}

pub type Spanned<T, L, E> = Result<(L, T, L), E>;
pub type LexerResult = Spanned<Token, Location, UnexpectedCharError>;

pub struct Lexer<'i> {
    input: Peekable<CharIndices<'i>>,
    location: Location
}

impl<'i> Lexer<'i> {
    pub fn new(input: &'i str) -> Self {
        Lexer {
            input: input.char_indices().peekable(),
            location: Default::default()
        }
    }

    pub fn peek(&mut self) -> Option<char> {
        self.input
            .peek()
            .cloned()
            .map(|(_, c)| c)
    }

    pub fn consume(&mut self) -> Option<char> {
        self.input
            .next()
            .map(|(_, c)| {
                match c {
                    '\n' => self.location.next_line(),
                    _ => self.location.next()
                }
                c
            })
    }

    fn consume_while<'a, P>(&'a mut self, predicate: P) -> ConsumeWhile<'a, 'i, P>
    where P: FnMut(char) -> bool {
        ConsumeWhile {
            lexer: self,
            predicate,
            done: false
        }
    }

    pub fn next_inner(&mut self, c: char) -> Result<Token, UnexpectedCharError> {
        macro_rules! token {
            ($x:path) => {{
                self.consume();
                Ok($x)
            }}
        }

        macro_rules! token_case {
            {def => $z:path $(,$x:pat => $y:path)+} => {{
                self.consume();
                match self.peek() {
                    $(
                        Some($x) => token!($y),
                    )+
                    _ => Ok($z)
                }
            }}
        }

        match c {
            ',' => token!(Token::Comma),
            ';' => token!(Token::Semi),
            '{' => token!(Token::BraceOpen),
            '}' => token!(Token::BraceClose),
            '(' => token!(Token::ParenOpen),
            ')' => token!(Token::ParenClose),
            '[' => token!(Token::BracketOpen),
            ']' => token!(Token::BracketClose),
            '=' => token!(Token::Equals),
            ':' => token!(Token::Colon),
            '+' => token!(Token::Plus),
            '*' => token!(Token::Star),
            '%' => token!(Token::Percent),
            '~' => token!(Token::Tilde),
            '!' => token!(Token::Exclamation),
            '&' => token!(Token::Ampersand),
            '-' => token_case!{
                def => Token::Minus,
                '>' => Token::Arrow
            },
            '/' => {
                self.consume();
                match self.peek() {
                    Some('/') => {
                        self.consume_while(|c| c != '\n').count(); // consume all
                        Ok(Token::Comment)
                    },
                    _ => Ok(Token::Slash)
                }
            },
            c if c.is_alphabetic() || !c.is_ascii() || c == '_' => {
                let word = self
                    .consume_while(|c| c.is_alphanumeric() || !c.is_ascii() || c == '_' || c == '?')
                    .collect::<String>();

                let tok = KEYWORDS.get(word.as_str())
                    .cloned()
                    .unwrap_or(Token::Ident(word));

                Ok(tok)
            },
            c if c.is_whitespace() => {
                self.consume_while(|c| c.is_whitespace()).count();
                Ok(Token::Whitespace)
            },
            c if c.is_digit(10) => {
                let word = self.consume_while(|c| c.is_digit(10))
                    .collect::<String>();
                Ok(Token::Integer(word))
            },
            _ => Err(UnexpectedCharError(c))
        }
    }

    pub fn next_token(&mut self) -> Option<LexerResult> {
        self.peek()
            .map(|c| {
                let start = self.location.clone();
                self.next_inner(c)
                    .map(|tok| (start, tok, self.location.clone()))
            }) 
    }

    pub fn next_filtered(&mut self) -> Option<LexerResult> {
        loop {
            match self.next_token() {
                Some(Ok((_, Token::Whitespace, _))) | Some(Ok((_, Token::Comment, _))) => continue,
                rest => return rest
            }
        }
    }
}

impl<'i> Iterator for Lexer<'i> {
    type Item = LexerResult;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_filtered()   
    }
}

#[derive(Debug)]
pub struct UnexpectedCharError(char);

impl fmt::Display for UnexpectedCharError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unexpected character '{}'", self.0)
    }
}

impl Error for UnexpectedCharError {
    fn description(&self) -> &str {
        "Unexpected character"
    }
}

struct ConsumeWhile<'a, 'i, P>
where 'i: 'a,
      P: FnMut(char) -> bool {
    lexer: &'a mut Lexer<'i>,
    predicate: P,
    done: bool
}

impl<'a, 'i, P> Iterator for ConsumeWhile<'a, 'i, P>
where 'i: 'a,
      P: FnMut(char) -> bool {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            match self.lexer.peek() {
                Some(c) => {
                    if (self.predicate)(c) {
                        Some(self.lexer.consume().unwrap())
                    } else {
                        self.done = true;
                        None
                    }
                },
                None => {
                    self.done = true;
                    None
                }
            }
        }
    }
}

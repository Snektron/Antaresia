pub mod lexer;
pub mod token;

use std::io::Read;
use std::fmt;
use std::error;
use std::iter::{Iterator, Peekable};
use parser::token::{Token, Keyword};
use parser::lexer::Lexer;
use ast::{Expr, BinOpKind, UnOpKind, Stmt, Program, Field, DataType};

struct Transformator<R>
where R: Read {
    lexer: Lexer<R>
}

impl<R> Transformator<R>
where R: Read {
    fn new(lexer: Lexer<R>) -> Transformator<R> {
        Transformator {
            lexer
        }
    }
}

impl<R> Iterator for Transformator<R>
where R: Read {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.lexer.next() {
                Some(Ok(Token::Whitespace)) | Some(Ok(Token::Comment(_))) => continue,
                Some(Ok(t)) => return Some(Ok(t)),
                Some(Err(e)) => return Some(Err(Error::Other(e))),
                None => return None
            };
        }
    }
}

pub struct Parser<R>
where R: Read {
    lexer: Peekable<Transformator<R>>
}

impl<R> Parser<R>
where R: Read {
    pub fn new(input: R) -> Parser<R> {
        Parser {
            lexer: Transformator::new(Lexer::new(input)).peekable()
        }
    }

    fn peek_inner(&mut self) -> Result<Option<&Token>, &Error> {
        match self.lexer.peek() {
            Some(&Ok(ref x)) => Ok(Some(x)),
            Some(&Err(ref e)) => Err(e),
            None => Ok(None)
        }
    }

    fn peek(&mut self) -> Result<Option<&Token>, Error> {
        match self.peek_inner() {
            Ok(_) => Ok(self.peek_inner().unwrap()),
            Err(_) => Err(self.lexer.next().unwrap().unwrap_err())
        }
    }

    fn consume(&mut self) -> Result<Option<Token>, Error> {
        match self.lexer.next() {
            Some(Ok(tok)) => Ok(Some(tok)),
            Some(Err(e)) => Err(e),
            None => Ok(None)
        }
    }

    fn next(&mut self) -> Result<Token, Error> {
        self.consume()
            .and_then(|tok| tok.ok_or(Error::UnexpectedEof))
    } 

    fn peek_next(&mut self) -> Result<&Token, Error> {
        self.peek()
            .and_then(|tok| tok.ok_or(Error::UnexpectedEof))
    }

    fn expect(&mut self, expected: Token) -> Result<(), Error> {
        self.next().and_then(|tok| {
            match tok {
                ref tok if tok == &expected => Ok(()),
                tok => Err(Error::UnexpectedToken(tok))
            }
        })
    }

    fn eat(&mut self, expected: Token) -> Result<bool, Error> {
        let matched = self.peek()
            .and_then(|tok| Ok(tok.is_some() && tok.unwrap() == &expected))?;
        if matched {
            self.consume()?;
        }

        Ok(matched)
    }

    fn ident(&mut self) -> Result<String, Error> {
        match self.next()? {
            Token::Ident(s) => Ok(s),
            tok => Err(Error::UnexpectedToken(tok))
        }
    }

    fn datatype(&mut self) -> Result<DataType, Error> {
        match self.next()? {
            Token::Ident(s) => Ok(DataType::Alias(s)),
            Token::DataType(dt) => Ok(dt),
            tok => Err(Error::UnexpectedToken(tok))
        }
    }

    pub fn program(&mut self) -> Result<Program, Error> {
        let mut stmts = Vec::<Box<Stmt>>::new();

        loop {
            match self.peek()? {
                Some(_) => stmts.push(Box::new(self.stmt()?)),
                None => return Ok(Program(stmts)),
            }
        }
    }

    fn stmt(&mut self) -> Result<Stmt, Error> {
        match self.peek_next()? {
            &Token::BraceOpen => self.block(),
            &Token::Keyword(Keyword::If) => self.ifstmt(),
            &Token::Keyword(Keyword::While) => self.whilestmt(),
            &Token::Keyword(Keyword::Func) => self.funcdecl(),
            &Token::Keyword(Keyword::Struct) => self.structdecl(),
            &Token::Keyword(Keyword::Return) => {
                self.consume()?;
                let expr = Box::new(self.expr()?);
                self.expect(Token::Semicolon)?;
                Ok(Stmt::Return(expr))
            },
            _ => {
                let expr = Box::new(self.expr()?);
                self.expect(Token::Semicolon)?;
                Ok(Stmt::Expr(expr))
            },
        }
    }

    fn whilestmt(&mut self) -> Result<Stmt, Error> {
        self.expect(Token::Keyword(Keyword::While))?;

        let condition = Box::new(self.expr()?);
        let consequent = Box::new(self.block()?);

        Ok(Stmt::While(condition, consequent))
    }

    fn ifstmt(&mut self) -> Result<Stmt, Error> {
        self.expect(Token::Keyword(Keyword::If))?;

        let condition = Box::new(self.expr()?);
        let consequent = Box::new(self.block()?);

        match self.peek()? {
            Some(&Token::Keyword(Keyword::Else)) => {
                self.consume()?;

                let alternative = match self.peek()? {
                    Some(&Token::Keyword(Keyword::If)) => self.ifstmt(),
                    _ => self.block()

                }?;
                    
                Ok(Stmt::If(condition, consequent, Some(Box::new(alternative))))
            },
            _ => Ok(Stmt::If(condition, consequent, None)),
        }
    }

    fn funcdecl(&mut self) -> Result<Stmt, Error> {
        self.expect(Token::Keyword(Keyword::Func))?;
        let name = self.ident()?;
        let params = self.params()?;

        let return_type = match self.peek()? {
            Some(&Token::Arrow) => {
                self.consume()?;

                match self.next()? {
                    Token::Ident(s) => Ok(DataType::Alias(s)),
                    Token::DataType(dt) => Ok(dt),
                    tok => Err(Error::UnexpectedToken(tok))
                }?
            },
            _ => DataType::Void
        };

        let body = Box::new(self.block()?);

        Ok(Stmt::FuncDecl(Field(return_type, name), params, body))
    }

    fn structdecl(&mut self) -> Result<Stmt, Error> {
        self.expect(Token::Keyword(Keyword::Struct))?;
        let name = self.ident()?;
        self.expect(Token::BraceOpen)?;

        let fields = match self.peek_next()? {
            &Token::BraceClose => self.consume().and(Ok(Vec::new())),
            _ => {
                let fields = self.fieldlist()?;
                self.expect(Token::BraceClose)?;
                Ok(fields)
            }
        }?;

        Ok(Stmt::StructDecl(name, fields))
    }

    fn params(&mut self) -> Result<Vec<Field>, Error> {
        self.expect(Token::ParenOpen)?;

        match self.peek_next()? {
            &Token::ParenClose => self.consume().and(Ok(Vec::new())),
            _ => {
                let fields = self.fieldlist()?;
                self.expect(Token::ParenClose)?;
                Ok(fields)
            }
        }
    }

    fn fieldlist(&mut self) -> Result<Vec<Field>, Error> {
        let mut fields = Vec::<Field>::new();

        let mut last_type = self.datatype()?;
        let name = self.ident()?;

        fields.push(Field(last_type.clone(), name));

        loop {
            if !self.eat(Token::Comma)? {
                return Ok(fields);
            }

            match self.next()? {
                Token::DataType(dt) => {
                    last_type = dt.clone();
                    let name = self.ident()?;

                    fields.push(Field(dt, name));
                },
                Token::Ident(first) => match self.peek()? {
                    Some(&Token::Ident(_)) => {
                        let second = self.ident()?;
                        last_type = DataType::Alias(first.clone());
                        fields.push(Field(DataType::Alias(first), second));
                    },
                    _ => fields.push(Field(last_type.clone(), first))
                },
                tok => return Err(Error::UnexpectedToken(tok))
            };
        }
    }

    fn block(&mut self) -> Result<Stmt, Error> {
        self.expect(Token::BraceOpen)?;
        let mut stmts = Vec::<Stmt>::new();

        loop {
            match self.peek_next()? {
                &Token::BraceClose => {
                    self.consume()?;
                    return Ok(match stmts.len() {
                        1 => stmts.pop().unwrap(),
                        _ => Stmt::Compound(stmts.into_iter().map(Box::new).collect())
                    })
                }
                _ => stmts.push(self.stmt()?),
            }
        }
    }

    fn expr(&mut self) -> Result<Expr, Error> {
        self.sum()
    }

    fn sum(&mut self) -> Result<Expr, Error> {
        let mut lhs = self.product()?;

        loop {
            match self.peek()? {
                Some(&Token::Plus) => {
                    self.consume()?;
                    let rhs = self.product()?;
                    lhs = Expr::Binary(BinOpKind::Add, Box::new(lhs), Box::new(rhs))
                },
                Some(&Token::Minus) => {
                    self.consume()?;
                    let rhs = self.product()?;
                    lhs = Expr::Binary(BinOpKind::Sub, Box::new(lhs), Box::new(rhs))
                },
                _ => return Ok(lhs),
            }
        }
    }

    fn product(&mut self) -> Result<Expr, Error> {
        let mut lhs = self.unary()?;

        loop {
            match self.peek()? {
                Some(&Token::Star) => {
                    self.consume()?;
                    lhs = Expr::Binary(BinOpKind::Mul, Box::new(lhs), Box::new(self.unary()?))
                },
                Some(&Token::Slash) => {
                    self.consume()?;
                    lhs = Expr::Binary(BinOpKind::Div, Box::new(lhs), Box::new(self.unary()?))
                },
                Some(&Token::Percent) => {
                    self.consume()?;
                    lhs = Expr::Binary(BinOpKind::Mod, Box::new(lhs), Box::new(self.unary()?))
                },
                _ => return Ok(lhs),
            }
        }
    }

    fn unary(&mut self) -> Result<Expr, Error> {
        match self.peek()? {
            Some(&Token::Minus) => {
                self.consume()?;
                Ok(Expr::Unary(UnOpKind::Neg, Box::new(self.unary()?)))
            },
            Some(&Token::Tilde) => {
                self.consume()?;
                Ok(Expr::Unary(UnOpKind::Compl, Box::new(self.unary()?)))
            },
            Some(&Token::Exclamation) => {
                self.consume()?;
                Ok(Expr::Unary(UnOpKind::Not, Box::new(self.unary()?)))
            },
            _ => self.atom(),
        }
    }

    fn atom(&mut self) -> Result<Expr, Error> {
        match self.next()? {
            Token::Literal(x) => Ok(Expr::Literal(x)),
            Token::ParenOpen => self.expr().and_then(|expr|
                self.expect(Token::ParenClose).and(Ok(expr))
            ),
            tok => Err(Error::UnexpectedToken(tok))
        }
    }
}

#[derive(Debug)]
pub enum Error {
    UnexpectedToken(Token),
    UnexpectedEof,
    Other(lexer::Error)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::UnexpectedToken(ref tok) => write!(f, "Unexpected token {:?}", tok),
            Error::UnexpectedEof => write!(f, "Unexpected end of file"),
            Error::Other(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::UnexpectedToken(_) => "Unexpected token",
            Error::UnexpectedEof => "Unexpected end of file",
            Error::Other(_) => "Other",
        }
    }
}

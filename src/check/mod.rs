pub mod typecheck;
pub mod environment;

use std::error::Error;
use std::fmt;
use ast::{Name, BinOpKind, UnOpKind};
use ast::ty::Ty;
use parser::Span;

pub type CheckResult<T> = Result<T, SemanticError>;

pub trait CheckType: Clone + PartialEq {
    type ExprInfo;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Checked;

impl CheckType for Checked {
    type ExprInfo = Ty<Checked>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Unchecked;

impl CheckType for Unchecked {
    type ExprInfo = ();
}

#[derive(Debug)]
pub struct SemanticError {
    span: Span,
    kind: SemanticErrorKind
}

impl SemanticError {
    pub fn new(span: Span, kind: SemanticErrorKind) -> Self {
        SemanticError {
            span,
            kind
        }
    }
}

impl Error for SemanticError {
    fn description(&self) -> &'static str {
        match self.kind {
            SemanticErrorKind::Redefinition(..) => "redefinition",
            SemanticErrorKind::Undefined(..) => "undefined",
            SemanticErrorKind::IllegalStatement => "illegal statement",
            SemanticErrorKind::TypeError(..) => "type error",
            SemanticErrorKind::InvalidBinary(..) => "invalid operands for binary operator",
            SemanticErrorKind::InvalidUnary(..) => "invalid operands for unary operator",
            SemanticErrorKind::NotAFunction(..) => "not a function"
        }
    }
}

#[derive(Debug)]
pub enum SemanticErrorKind {
    Redefinition(Span, Name),
    Undefined(Name),
    IllegalStatement,
    TypeError(Ty<Checked>, Ty<Checked>),
    InvalidBinary(BinOpKind, Ty<Checked>, Ty<Checked>),
    InvalidUnary(UnOpKind, Ty<Checked>),
    NotAFunction(Ty<Checked>)
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: ", self.span.0)?;
        match self.kind {
            SemanticErrorKind::Redefinition(ref origin, ref name) => {
                write!(f, "Redefinition: '{}' is already defined at {}", name, origin.0)
            },
            SemanticErrorKind::Undefined(ref name) => {
                write!(f, "Undefined variable or type '{}'", name)
            },
            SemanticErrorKind::IllegalStatement => {
                write!(f, "Illegal statement: statement is not allowed here")
            },
            SemanticErrorKind::TypeError(ref expected, ref actual) => {
                write!(f, "Type error: expected '{}', found '{}'", expected, actual)
            },
            SemanticErrorKind::InvalidBinary(ref op, ref lhs, ref rhs) => {
                write!(f, "Invalid arguments '{}' and '{}' to binary operator {}", lhs, rhs, op)
            },
            SemanticErrorKind::InvalidUnary(ref op, ref lhs) => {
                write!(f, "Invalid argument '{}' to unary operator {}", lhs, op)
            },
            SemanticErrorKind::NotAFunction(ref actual) => {
                write!(f, "Not a function: expected function, found '{}'", actual)
            }
        }
    }
}


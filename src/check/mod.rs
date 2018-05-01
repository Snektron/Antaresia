pub mod typecheck;

use std::error::Error;
use std::fmt;
use ast::{Name, BinOpKind, UnOpKind};
use ast::ty::Ty;
use parser::Span;

pub type CheckResult<T> = Result<T, SemanticError>;

pub trait CheckType {
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
            SemanticErrorKind::InvalidReturnType(..) => "invalid return type",
            SemanticErrorKind::InvalidBinary(..) => "invalid operands for binary operator",
            SemanticErrorKind::InvalidUnary(..) => "invalid operands for unary operator",
        }
    }
}

#[derive(Debug)]
pub enum SemanticErrorKind {
    Redefinition(Span, Name),
    Undefined(Name),
    IllegalStatement,
    InvalidReturnType(Ty<Checked>, Ty<Checked>),
    InvalidBinary(BinOpKind, Ty<Checked>, Ty<Checked>),
    InvalidUnary(UnOpKind, Ty<Checked>),
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            SemanticErrorKind::Redefinition(ref origin, ref name) => {
                write!(f, "{}: Redefinition error: '{}' is already defined at {}", self.span.0, name, origin.0)
            },
            SemanticErrorKind::Undefined(ref name) => {
                write!(f, "{}: Undefined variable or type '{}'", self.span.0, name)
            },
            SemanticErrorKind::IllegalStatement => {
                write!(f, "{}: Illegal statement error: statement is not allowed here", self.span.0)
            },
            SemanticErrorKind::InvalidReturnType(ref expected, ref actual) => {
                write!(f, "{}: Invalid return type: expected '{}' as defined at {}, found '{}'", self.span.0, expected, expected.span.0, actual)
            },
            SemanticErrorKind::InvalidBinary(ref op, ref lhs, ref rhs) => {
                write!(f, "{}: Invalid arguments '{}' and '{}' to binary operator {}", self.span.0, lhs, rhs, op)
            },
            SemanticErrorKind::InvalidUnary(ref op, ref lhs) => {
                write!(f, "{}: Invalid argument '{}' to unary operator {}", self.span.0, lhs, op)
            },
        }
    }
}


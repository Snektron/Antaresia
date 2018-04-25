pub mod scoped;
pub mod context;
mod check;

pub use self::context::{Context, Frame};

use std::error::Error;
use std::fmt;
use ast::Name;
use ast::DataType;
use parser::Span;

pub type CheckResult<T> = Result<T, SemanticError>;

pub trait CheckType {
    type ExprInfo;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Checked;

impl CheckType for Checked {
    type ExprInfo = DataType<Checked>;
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
            SemanticErrorKind::OutOfScope(_) => "out-of-scope",
            SemanticErrorKind::IllegalStatement => "illegal statement",
            SemanticErrorKind::InvalidReturnType(..) => "invalid return type"
        }
    }
}

#[derive(Debug)]
pub enum SemanticErrorKind {
    Redefinition(Span, Name),
    OutOfScope(Name),
    IllegalStatement,
    InvalidReturnType(DataType, DataType)
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            SemanticErrorKind::Redefinition(ref origin, ref name)
                => write!(f, "{}: Redefinition error: '{}' is already defined at {}", self.span.0, name, origin.0),
            SemanticErrorKind::OutOfScope(ref name)
                => write!(f, "{}: Out-of-scope error: '{}' was not found in this", self.span.0, name),
            SemanticErrorKind::IllegalStatement
                => write!(f, "{}: Illegal statement error: statement is not allowed here", self.span.0),
            SemanticErrorKind::InvalidReturnType(ref expected, ref actual)
                => write!(f, "{}: Invalid return type: expected {} as defined on {}, found {}", self.span.0, expected, expected.span.0, actual)
        }
    }
}


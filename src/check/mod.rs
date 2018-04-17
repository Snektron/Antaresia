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

#[derive(Clone)]
pub struct Checked;

impl CheckType for Checked {
    type ExprInfo = DataType;
}

#[derive(Clone)]
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

#[derive(Debug)]
pub enum SemanticErrorKind {
    Redefinition(Span, Name),
    OutOfScope(Name)
}

impl Error for SemanticError {
    fn description(&self) -> &'static str {
        match self.kind {
            SemanticErrorKind::Redefinition(_, _) => "redefinition error",
            SemanticErrorKind::OutOfScope(_) => "out-of-scope error"
        }
    }
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            SemanticErrorKind::Redefinition(ref origin, ref name)
                => write!(f, "{}: Redefinition error: '{}' is already defined at {}.", self.span.0, name, origin.0),
            SemanticErrorKind::OutOfScope(ref name)
                => write!(f, "{}: Out-of-scope error: '{}' was not found in this.", self.span.0, name)
        }
    }
}


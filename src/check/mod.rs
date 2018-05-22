use std::error::Error;
use std::hash::Hash;
use std::fmt;
use ast::ty::Ty;
use parser::Span;

macro_rules! fmt_err {
    ($span:expr, $($arg:tt)*) => {
        ::check::SemanticError {
            span: $span,
            message: format!($($arg)*)
        }
    }
}

macro_rules! err {
    ($span:expr, $($arg:tt)*) => {
        Err(fmt_err!($span, $($arg)*))
    }
}

//pub mod typecheck;
//pub mod environment;
//pub mod scope;
pub mod scope_tree;

pub trait CheckType: Clone + PartialEq + Eq + Hash {
    type ExprInfo: Clone;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Checked;

impl CheckType for Checked {
    type ExprInfo = Ty<Checked>;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Unchecked;

impl CheckType for Unchecked {
    type ExprInfo = ();
}

#[derive(Debug)]
pub struct SemanticError {
    span: Span,
    message: String
}

impl Error for SemanticError {
    fn description(&self) -> &str {
        self.message.as_str()
    }
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: Error: {}", self.span, self.message)
    }
}

pub type CheckResult<T> = Result<T, SemanticError>;

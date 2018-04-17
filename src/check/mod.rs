pub mod scoped;
pub mod context;
mod check;

pub use self::context::{Context, Frame};

use std::error::Error;
use std::fmt;
use ast::Name;
use ast::DataType;

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
pub enum SemanticError {
    Redefinition(Name),
    OutOfScope(Name)
}

impl Error for SemanticError {
    fn description(&self) -> &'static str {
        match *self {
            SemanticError::Redefinition(_) => "redefinition error",
            SemanticError::OutOfScope(_) => "out-of-scope error"
        }
    }
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SemanticError::Redefinition(ref name)
                => write!(f, "Redefinition error: '{}' is already defined.", name),
            SemanticError::OutOfScope(ref name)
                => write!(f, "Out-of-scope error: '{}' was not found in this.", name)
        }
    }
}


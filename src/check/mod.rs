pub mod scoped;
pub mod context;
mod check;

pub use self::context::{Context, Frame};

use std::error::Error;
use std::fmt;
use ast::Name;
use datatype::DataType;

pub trait CheckType {
    type ExprInfo;
}

pub struct Checked {}

impl CheckType for Checked {
    type ExprInfo = DataType;
}

pub struct Unchecked {}

impl CheckType for Unchecked {
    type ExprInfo = ();
}

impl Default for Unchecked {
    fn default() -> Self {
        Unchecked {}
    }
}

#[derive(Debug)]
pub enum SemanticError {
    Redefinition(Name)
}

impl Error for SemanticError {
    fn description(&self) -> &'static str {
        match *self {
            SemanticError::Redefinition(_) => "redefinition error"
        }
    }
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SemanticError::Redefinition(ref name)
                => write!(f, "Redefinition error: '{}' is already defined.", name)
        }
    }
}


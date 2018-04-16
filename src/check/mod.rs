pub mod scoped;
pub mod context;
mod check;

pub use self::context::{Context, Frame};

use std::error::Error;
use std::fmt;
use ast::Name;
use datatype::DataType;

pub trait StmtInfo {
    type ExprInfo: ExprInfo;
}

pub struct CheckedStmtInfo {}

impl StmtInfo for CheckedStmtInfo {
    type ExprInfo = CheckedExprInfo;
}

pub struct UncheckedStmtInfo {}

impl Default for UncheckedStmtInfo {
    fn default() -> Self {
        UncheckedStmtInfo {}
    }
}

impl StmtInfo for UncheckedStmtInfo {
    type ExprInfo = UncheckedExprInfo;
}

pub trait ExprInfo {}

pub struct UncheckedExprInfo {}

impl ExprInfo for UncheckedExprInfo {}

impl Default for UncheckedExprInfo {
    fn default() -> Self {
        UncheckedExprInfo {}
    }
}

pub struct CheckedExprInfo {
    datatype: DataType
}

impl ExprInfo for CheckedExprInfo {}

pub trait Check {
    type Target;

    fn check<'s>(self, ctx: &mut Context<'s>) -> Result<Self::Target, SemanticError>;
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


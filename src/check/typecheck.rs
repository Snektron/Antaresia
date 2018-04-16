use std::error::Error;
use std::fmt;
use check::scoped_map::ScopedMap;
use check::{CheckStmt, UncheckedStmt, CheckedStmt};
use check::{CheckExpr, UncheckedExpr, CheckedExpr};
use datatype::{DataType, Field};
use ast::Name;
use ast::{Program, Stmt, StmtKind, Expr, ExprKind};

pub struct TypeChecker {
    scope: ScopedMap<Name, DataType>
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            scope: ScopedMap::new()
        }
    }
}

pub trait Check {
    type Target;

    fn check(self, tc: &mut TypeChecker) -> Result<Self::Target, TypeError>;
}

impl Check for Program<UncheckedStmt> {
    type Target = Program<CheckedStmt>;

    fn check(self, tc: &mut TypeChecker) -> Result<Self::Target, TypeError> {
        Err(TypeError::Redefinition("test".to_owned()))
    }
}

impl Check for Stmt<UncheckedStmt> {
    type Target = Stmt<CheckedStmt>;

    fn check(self, tc: &mut TypeChecker) -> Result<Self::Target, TypeError> {
        Err(TypeError::Redefinition("test".to_owned()))
    }
}

impl Check for Expr<UncheckedExpr> {
    type Target = Expr<CheckedExpr>;

    fn check(self, tc: &mut TypeChecker) -> Result<Self::Target, TypeError> {
        Err(TypeError::Redefinition("test".to_owned()))
    }
}

#[derive(Debug)]
pub enum TypeError {
    Redefinition(Name)
}

impl Error for TypeError {
    fn description(&self) -> &'static str {
        match *self {
            TypeError::Redefinition(_) => "redefinition error"
        }
    }
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TypeError::Redefinition(ref name) => write!(f, "Redefinition error: '{}' is already defined.", name)
        }
    }
}


use std::error::Error;
use std::fmt;
use check::scoped_map::ScopedMap;
use check::{StmtInfo, UncheckedStmtInfo, CheckedStmtInfo};
use check::{ExprInfo, UncheckedExprInfo, CheckedExprInfo};
use datatype::DataType;
use ast::Name;
use ast::{Program, Stmt, Expr};

pub struct TypeChecker {
    bindings: ScopedMap<Name, DataType>,
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker {
            bindings: ScopedMap::new()
        }
    }

    pub fn enter(&mut self) {
        self.bindings.enter();
    //    self.types.enter();
    }

    pub fn exit(&mut self) {
        self.bindings.exit();
    //    self.types.exit();
    }
}

pub trait Check {
    type Target;

    fn check(self, tc: &mut TypeChecker) -> Result<Self::Target, TypeError>;
}

impl Check for Program {
    type Target = Program<CheckedStmtInfo>;

    fn check(self, tc: &mut TypeChecker) -> Result<Self::Target, TypeError> {
        Err(TypeError::Redefinition("test".to_owned()))
    }
}

impl Check for Stmt {
    type Target = Stmt<CheckedStmtInfo>;

    fn check(self, tc: &mut TypeChecker) -> Result<Self::Target, TypeError> {
        Err(TypeError::Redefinition("test".to_owned()))
    }
}

impl Check for Expr {
    type Target = Expr<CheckedExprInfo>;

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


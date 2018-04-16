pub mod scoped_map;
pub mod typecheck;

pub use self::typecheck::{TypeChecker, Check};

use datatype::DataType;
use std::default::Default;

pub trait CheckStmt {
    type Expr: CheckExpr;
}

pub struct CheckedStmt {}

impl CheckStmt for CheckedStmt {
    type Expr = CheckedExpr;
}

pub struct UncheckedStmt {}

impl Default for UncheckedStmt {
    fn default() -> Self {
        UncheckedStmt {}
    }
}

impl CheckStmt for UncheckedStmt {
    type Expr = UncheckedExpr;
}

pub trait CheckExpr {}

pub struct UncheckedExpr {}

impl CheckExpr for UncheckedExpr {}

impl Default for UncheckedExpr {
    fn default() -> Self {
        UncheckedExpr {}
    }
}

pub struct CheckedExpr {
    datatype: DataType
}

impl CheckExpr for CheckedExpr {}

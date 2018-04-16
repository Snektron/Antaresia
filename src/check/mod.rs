pub mod scoped_map;
pub mod typecheck;

pub use self::typecheck::{TypeChecker, Check};

use datatype::DataType;
use std::default::Default;

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

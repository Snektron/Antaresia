pub mod print;
pub mod expr;

pub use self::expr::{Expr, ExprKind, Literal, BinOpKind, UnOpKind};
pub use self::print::print;

use std::default::Default;
use datatype::{DataType, Field};
use check::{StmtInfo, UncheckedStmtInfo};

pub type Name = String;

pub struct Program<I = UncheckedStmtInfo>
where I: StmtInfo {
    pub stmts: Vec<Stmt<I>>
}

impl<I> Program<I>
where I: StmtInfo {
    pub fn new(stmts: Vec<Stmt<I>>) -> Self {
        Program {
            stmts
        }
    }
}

pub struct Stmt<I = UncheckedStmtInfo>
where I: StmtInfo {
    pub kind: StmtKind<I>,
    pub check: I
}

impl Stmt<UncheckedStmtInfo> {
    pub fn new(kind: StmtKind<UncheckedStmtInfo>) -> Self {
        Stmt {
            kind,
            check: Default::default()
        }
    }
}

pub enum StmtKind<I = UncheckedStmtInfo>
where I: StmtInfo {
    Compound(Vec<Stmt<I>>),
    If(Box<Expr<I::ExprInfo>>, Box<Stmt<I>>, Option<Box<Stmt<I>>>),
    While(Box<Expr<I::ExprInfo>>, Box<Stmt<I>>),
    Return(Box<Expr<I::ExprInfo>>),
    Expr(Box<Expr<I::ExprInfo>>),
    FuncDecl(Name, DataType, Vec<Field>, Box<Stmt<I>>),
    StructDecl(Name, Vec<Field>)
}

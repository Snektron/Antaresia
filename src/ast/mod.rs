pub mod print;
pub mod expr;

pub use self::expr::{Expr, ExprKind, Literal, BinOpKind, UnOpKind};
pub use self::print::print;

use std::default::Default;
use datatype::{DataType, Field};
use check::{CheckStmt, UncheckedStmt};

pub type Name = String;

pub struct Program<C = UncheckedStmt>
where C: CheckStmt {
    pub stmts: Vec<Stmt<C>>
}

impl<C> Program<C>
where C: CheckStmt {
    pub fn new(stmts: Vec<Stmt<C>>) -> Self {
        Program {
            stmts
        }
    }
}

pub struct Stmt<C = UncheckedStmt>
where C: CheckStmt {
    pub kind: StmtKind<C>,
    pub check: C
}

impl Stmt<UncheckedStmt> {
    pub fn new(kind: StmtKind<UncheckedStmt>) -> Self {
        Stmt {
            kind,
            check: Default::default()
        }
    }
}

pub enum StmtKind<C = UncheckedStmt>
where C: CheckStmt {
    Compound(Vec<Stmt<C>>),
    If(Box<Expr<C::Expr>>, Box<Stmt<C>>, Option<Box<Stmt<C>>>),
    While(Box<Expr<C::Expr>>, Box<Stmt<C>>),
    Return(Box<Expr<C::Expr>>),
    Expr(Box<Expr<C::Expr>>),
    FuncDecl(Name, DataType, Vec<Field>, Box<Stmt<C>>),
    StructDecl(Name, Vec<Field>)
}

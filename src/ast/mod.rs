pub mod print;
pub mod expr;

pub use self::expr::{Expr, ExprKind, Literal, BinOpKind, UnOpKind};
pub use self::print::print;

use datatype::{DataType, Field};
use check::{CheckType, Unchecked};

pub type Name = String;

pub struct Program<C = Unchecked>
where C: CheckType {
    pub stmts: Vec<Stmt<C>>
}

impl<C> Program<C>
where C: CheckType {
    pub fn new(stmts: Vec<Stmt<C>>) -> Self {
        Program {
            stmts
        }
    }
}

pub struct Stmt<C = Unchecked>
where C: CheckType {
    pub kind: StmtKind<C>,
}

impl<C> Stmt<C>
where C: CheckType {
    pub fn new(kind: StmtKind<C>) -> Self {
        Stmt {
            kind
        }
    }
}

pub enum StmtKind<C = Unchecked>
where C: CheckType {
    Compound(Vec<Stmt<C>>),
    If(Box<Expr<C>>, Box<Stmt<C>>, Option<Box<Stmt<C>>>),
    While(Box<Expr<C>>, Box<Stmt<C>>),
    Return(Box<Expr<C>>),
    Expr(Box<Expr<C>>),
    FuncDecl(Name, DataType, Vec<Field>, Box<Stmt<C>>),
    StructDecl(Name, Vec<Field>)
}

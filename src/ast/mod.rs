pub mod print;
pub mod expr;
pub mod datatype;

pub use self::expr::{Expr, ExprKind, Literal, BinOpKind, UnOpKind};
pub use self::datatype::{DataType, DataTypeKind, Field};

use check::{CheckType, Unchecked};
use parser::Span;

pub type Name = String;

pub struct Program<C = Unchecked>
where C: CheckType {
    pub span: Span,
    pub stmts: Vec<Stmt<C>>
}

impl<C> Program<C>
where C: CheckType {
    pub fn new(span: Span, stmts: Vec<Stmt<C>>) -> Self {
        Program {
            span,
            stmts
        }
    }
}

pub struct Stmt<C = Unchecked>
where C: CheckType {
    pub span: Span,
    pub kind: StmtKind<C>,
}

impl<C> Stmt<C>
where C: CheckType {
    pub fn new(span: Span, kind: StmtKind<C>) -> Self {
        Stmt {
            span,
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
    FuncDecl(Name, Box<DataType<C>>, Vec<Field<C>>, Box<Stmt<C>>),
    StructDecl(Name, Vec<Field<C>>)
}

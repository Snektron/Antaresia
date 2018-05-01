pub mod print;
pub mod expr;
pub mod ty;

pub use self::expr::{Expr, ExprKind, Literal, BinOpKind, UnOpKind};

use check::{CheckType, Unchecked};
use parser::Span;
use ast::ty::{Ty, Field};

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

    pub fn is_global_legal(&self) -> bool {
        match self.kind {
            StmtKind::FuncDecl(..) => true,
            StmtKind::StructDecl(..) => true,
            StmtKind::Expr(ref expr) => expr.is_global_legal(),
            _ => false
        }
    }
}

pub type Stmts<C = Unchecked> = Vec<Stmt<C>>;

pub enum StmtKind<C = Unchecked>
where C: CheckType {
    Compound(Stmts<C>),
    If(Box<Expr<C>>, Box<Stmt<C>>, Option<Box<Stmt<C>>>),
    While(Box<Expr<C>>, Box<Stmt<C>>),
    Return(Box<Expr<C>>),
    Expr(Box<Expr<C>>),
    FuncDecl(Name, Box<Ty<C>>, Vec<Field<C>>, Box<Stmt<C>>),
    StructDecl(Name, Vec<Field<C>>)
}

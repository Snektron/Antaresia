use std::default::Default;
use datatype::{DataType, Field};
use check::{CheckExpr, UncheckedExpr};
use ast::Name;

pub struct Expr<C = UncheckedExpr>
where C: CheckExpr {
    pub kind: ExprKind<C>,
    pub check: C
}

impl Expr<UncheckedExpr> {
    pub fn new(kind: ExprKind<UncheckedExpr>) -> Self {
        Expr {
            kind,
            check: Default::default()
        }
    }
}

pub enum ExprKind<C = UncheckedExpr>
where C: CheckExpr {
    Binary(BinOpKind, Box<Expr<C>>, Box<Expr<C>>),
    Unary(UnOpKind, Box<Expr<C>>),
    Call(Box<Expr<C>>, Vec<Expr<C>>),
    Subscript(Box<Expr<C>>, Box<Expr<C>>),
    Cast(Box<Expr<C>>, DataType),
    Literal(Literal),
    Name(Name),
    Decl(Field, Option<Box<Expr<C>>>),
}

#[derive(Debug)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Assign
}

#[derive(Debug)]
pub enum UnOpKind {
    Neg,
    Compl,
    Not,
    Deref,
    Ref
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Integer(i64)
}

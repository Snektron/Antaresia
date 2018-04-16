use std::default::Default;
use datatype::{DataType, Field};
use check::{ExprInfo, UncheckedExprInfo};
use ast::Name;

pub struct Expr<I = UncheckedExprInfo>
where I: ExprInfo {
    pub kind: ExprKind<I>,
    pub check: I
}

impl Expr<UncheckedExprInfo> {
    pub fn new(kind: ExprKind<UncheckedExprInfo>) -> Self {
        Expr {
            kind,
            check: Default::default()
        }
    }
}

pub enum ExprKind<I = UncheckedExprInfo>
where I: ExprInfo {
    Binary(BinOpKind, Box<Expr<I>>, Box<Expr<I>>),
    Unary(UnOpKind, Box<Expr<I>>),
    Call(Box<Expr<I>>, Vec<Expr<I>>),
    Subscript(Box<Expr<I>>, Box<Expr<I>>),
    Cast(Box<Expr<I>>, DataType),
    Literal(Literal),
    Name(Name),
    Decl(Field, Option<Box<Expr<I>>>),
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

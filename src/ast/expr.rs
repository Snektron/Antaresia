use std::fmt;
use std::default::Default;
use ast::{Name};
use ast::ty::{Ty, TyKind, Field};
use check::{CheckType, Unchecked, Checked};
use parser::Span;

pub struct Expr<C = Unchecked>
where C: CheckType {
    pub span: Span,
    pub kind: ExprKind<C>,
    pub info: C::ExprInfo
}

impl<C> Expr<C>
where C: CheckType {
    pub fn new(span: Span, kind: ExprKind<C>, info: C::ExprInfo) -> Self {
        Expr {
            span,
            kind,
            info
        }
    }

    pub fn is_global_legal(&self) -> bool {
        match self.kind {
            ExprKind::Decl(..) => true,
            ExprKind::Binary(BinOpKind::Assign, ..) => true,
            _ => false
        }
    }
}

impl Expr<Unchecked> {
    pub fn unchecked(span: Span, kind: ExprKind<Unchecked>) -> Self {
        Expr::new(span, kind, ())
    }
}

pub enum ExprKind<C = Unchecked>
where C: CheckType {
    Binary(BinOpKind, Box<Expr<C>>, Box<Expr<C>>),
    Unary(UnOpKind, Box<Expr<C>>),
    Call(Box<Expr<C>>, Vec<Expr<C>>),
    Subscript(Box<Expr<C>>, Box<Expr<C>>),
    Cast(Box<Expr<C>>, Ty<C>),
    Literal(Literal),
    Name(Name),
    Decl(Field<C>, Option<Box<Expr<C>>>),
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

impl fmt::Display for BinOpKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BinOpKind::Add => write!(f, "+"),
            BinOpKind::Sub => write!(f, "-"),
            BinOpKind::Mul => write!(f, "*"),
            BinOpKind::Div => write!(f, "/"),
            BinOpKind::Mod => write!(f, "%"),
            BinOpKind::Assign => write!(f, "="),
        }
    }
}

#[derive(Debug)]
pub enum UnOpKind {
    Neg,
    Compl,
    Not,
    Deref,
    Ref
}

impl fmt::Display for UnOpKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UnOpKind::Neg => write!(f, "-"),
            UnOpKind::Compl => write!(f, "~"),
            UnOpKind::Not => write!(f, "!"),
            UnOpKind::Deref => write!(f, "*"),
            UnOpKind::Ref => write!(f, "&"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Integer(i64)
}

impl Literal {
    pub fn ty(&self) -> Ty<Checked> {
        match *self {
            Literal::Integer(..) => Ty::new(Default::default(), TyKind::U8)
        }
    }
}
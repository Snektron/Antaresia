use ast::{Name, DataType, Field};
use check::{CheckType, Unchecked};
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
    Cast(Box<Expr<C>>, DataType<C>),
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

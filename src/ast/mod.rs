pub mod print;

use datatype::{DataType, Field};
use check::{Check, UnChecked};

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Integer(i64)
}

pub struct Program(pub Vec<Stmt>);

pub enum Stmt {
    Compound(Vec<Stmt>),
    If(Box<Expr>, Box<Stmt>, Option<Box<Stmt>>),
    While(Box<Expr>, Box<Stmt>),
    Return(Box<Expr>),
    Expr(Box<Expr>),
    FuncDecl(String, DataType, Vec<Field>, Box<Stmt>),
    StructDecl(String, Vec<Field>)
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

pub struct Expr<C = UnChecked>
where C: Check {
    pub kind: ExprKind<C>,
    pub check: C
}

impl Expr<UnChecked> {
    pub fn new(kind: ExprKind<UnChecked>) -> Self {
        Expr {
            kind,
            check: UnChecked {}
        }
    }
}

pub enum ExprKind<C = UnChecked>
where C: Check {
    Binary(BinOpKind, Box<Expr<C>>, Box<Expr<C>>),
    Unary(UnOpKind, Box<Expr<C>>),
    Call(Box<Expr<C>>, Vec<Expr<C>>),
    Subscript(Box<Expr<C>>, Box<Expr<C>>),
    Cast(Box<Expr<C>>, DataType),
    Literal(Literal),
    Name(String),
    Decl(Field, Option<Box<Expr<C>>>),
}
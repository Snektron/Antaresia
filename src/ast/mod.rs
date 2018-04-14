pub mod print;

#[derive(Clone, Debug, PartialEq)]
pub enum DataType {
    U8,
    Void,
    Alias(String),
    Ptr(Box<DataType>)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Integer(i64)
}

#[derive(Debug)]
pub struct Field(pub DataType, pub String);

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
    Deref
}

pub enum Expr {
    Binary(BinOpKind, Box<Expr>, Box<Expr>),
    Unary(UnOpKind, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
    Subscript(Box<Expr>, Box<Expr>),
    Cast(Box<Expr>, DataType),
    Literal(Literal),
    Name(String),
    Decl(Field, Option<Box<Expr>>),
}
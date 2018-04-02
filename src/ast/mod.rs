pub mod print;

#[derive(Clone, Debug, PartialEq)]
pub enum DataType {
    U8,
    Void,
    Alias(String)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Integer(i64)
}

#[derive(Debug)]
pub struct Field(pub DataType, pub String);

#[derive(Debug)]
pub struct Program(pub Vec<Box<Stmt>>);

#[derive(Debug)]
pub enum Stmt {
    Compound(Vec<Box<Stmt>>),
    If(Box<Expr>, Box<Stmt>, Option<Box<Stmt>>),
    While(Box<Expr>, Box<Stmt>),
    Return(Box<Expr>),
    Expr(Box<Expr>),
    FuncDecl(Field, Vec<Field>, Box<Stmt>),
    StructDecl(String, Vec<Field>)
}

#[derive(Debug)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
    Mod
}

#[derive(Debug)]
pub enum UnOpKind {
    Neg,
    Compl,
    Not,
}

#[derive(Debug)]
pub enum Expr {
    Binary(BinOpKind, Box<Expr>, Box<Expr>),
    Unary(UnOpKind, Box<Expr>),
    Literal(Literal)
}
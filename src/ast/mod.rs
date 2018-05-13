pub mod print;
pub mod expr;
pub mod ty;

use check::{CheckType, Unchecked};
use parser::Span;
use ast::ty::{Ty, Field, FuncTy};
use ast::expr::Expr;


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
            StmtKind::TypeDecl(..) => true,
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
    FuncDecl(Box<FuncDecl<C>>),
    TypeDecl(Box<TypeDecl<C>>)
}

pub struct FuncDecl<C = Unchecked>
where C: CheckType {
    pub name: Name,
    pub params: Vec<Field<C>>,
    pub return_ty: Ty<C>,
    pub body: Stmt<C>
}

impl<C> FuncDecl<C>
where C: CheckType {
    pub fn new(name: Name, params: Vec<Field<C>>, return_ty: Ty<C>, body: Stmt<C>) -> Self {
        Self {
            name,
            params,
            return_ty,
            body
        }
    }

    pub fn ty(&self) -> FuncTy<C> {
        let params = self.params
            .iter()
            .map(|field| &field.ty)
            .cloned()
            .collect();

        FuncTy::new(params, self.return_ty.clone())
    }
}

pub struct TypeDecl<C = Unchecked>
where C: CheckType {
    pub name: Name,
    pub ty: Ty<C>
}

impl<C> TypeDecl<C>
where C: CheckType {
    pub fn new(name: Name, ty: Ty<C>) -> Self {
        TypeDecl {
            name,
            ty
        }
    }
}
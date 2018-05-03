use std::collections::HashMap;
use utility::JoinExt;
use check::{Checked, CheckResult, SemanticError as SE, SemanticErrorKind as SEK};
use check::environment::Environment;
use ast::ty::{Ty, TyKind, Field, FuncTy};
use ast::{Name, Program};
use ast::{Stmt, Stmts, StmtKind};
use ast::{Expr, ExprKind, BinOpKind, UnOpKind};
use parser::Span;

type Struct = (Span, Vec<Field<Checked>>);

pub struct Frame {
    bindings: HashMap<Name, Ty<Checked>>,
    aliases: HashMap<Name, Struct>,
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            bindings: HashMap::new(),
            aliases: HashMap::new(),
        }
    }
}

pub struct Checker<'e> {
    env: Environment<'e>
}

impl<'e> Checker<'e> {
    pub fn new() -> Self {
        Checker {
            env: Environment::new()
        }
    }

    fn enter<'a>(&'a self) -> Checker<'a> {
        Checker {
            env: self.env.enter()
        }
    }

    pub fn forward_declare(&mut self, unchecked: &Stmts) -> CheckResult<()> {
        for stmt in unchecked {
            match stmt.kind {
                StmtKind::FuncDecl(ref decl) => {
                    let ty = self.enter()
                        .check_func_ty(decl.ty())
                        .map(|sig| Ty::new(stmt.span.clone(), TyKind::Func(Box::new(sig))));
                    // close scope
                    ty.and_then(|ty| self.env.declare_binding(decl.name.clone(), ty))?;
                },
                StmtKind::StructDecl(ref name, ref fields) => {
                    self.check_fields(fields.to_vec())
                        .and_then(|fields| self.env.declare_struct(name.clone(), (stmt.span.clone(), fields)))?;
                },
                _ => {}
            }
        }

        Ok(())
    }

    pub fn convert(&mut self, actual: Ty<Checked>, expected: &Ty<Checked>) -> CheckResult<Ty<Checked>> {
        Err(SE::new(actual.span.clone(), SEK::TypeError(expected.clone(), actual)))
    }

    pub fn check_program(&mut self, unchecked: Program) -> CheckResult<Program<Checked>> {
        self.forward_declare(&unchecked.stmts)?;

        let checked = unchecked.stmts
            .into_iter()
            .map(|stmt| {
                if stmt.is_global_legal() {
                    self.check_stmt(stmt)
                } else {
                    Err(SE::new(stmt.span, SEK::IllegalStatement))
                }
            })
            .collect::<CheckResult<_>>()?;

        Ok(Program::new(unchecked.span, checked))
    }

    pub fn check_stmt(&mut self, unchecked: Stmt) -> CheckResult<Stmt<Checked>> {
        Err(SE::new(unchecked.span, SEK::IllegalStatement))
    }

    pub fn check_stmts(&mut self, unchecked: Stmts) -> CheckResult<Stmts<Checked>> {
        self.forward_declare(&unchecked)?;

        unchecked
            .into_iter()
            .map(|stmt| self.check_stmt(stmt))
            .collect()
    }
    
    pub fn check_expr(&mut self, unchecked: Expr) -> CheckResult<Expr<Checked>> {
        let span = unchecked.span;
        let checked = match unchecked.kind {
            ExprKind::Binary(op, lhs, rhs) => {
                let lhs = self.check_expr(*lhs)?;
                let rhs = self.check_expr(*rhs)?;

                Err(SE::new(span.clone(), SEK::IllegalStatement))
            },
            ExprKind::Literal(lit) => {
                let ty = lit.ty();
                Ok((ExprKind::Literal(lit), ty))
            },
            ExprKind::Name(name) => {
                self.env
                    .get_binding(&name)
                    .cloned()
                    .ok_or(SE::new(span.clone(), SEK::Undefined(name.clone())))
                    .map(|ty| (ExprKind::Name(name), ty))
            },
            _ => {
                Err(SE::new(span.clone(), SEK::IllegalStatement))
            }
        };

        checked.map(|(kind, result)| Expr::new(span, kind, result))
    }

    pub fn check_expr_expected(&mut self, unchecked: Expr, expected: &Ty<Checked>) -> CheckResult<Expr<Checked>> {
        let expr = self.check_expr(unchecked)?;

        if expr.info != *expected {
            return Err(SE::new(expr.span, SEK::TypeError(expected.clone(), expr.info)))
        }

        Ok(expr)
    }

    pub fn check_exprs(&mut self, unchecked: Vec<Expr>) -> CheckResult<Vec<Expr<Checked>>> {
        unchecked
            .into_iter()
            .map(|expr| self.check_expr(expr))
            .collect()
    }

    pub fn check_ty(&self, unchecked: Ty) -> CheckResult<Ty<Checked>> {
        let span = unchecked.span;

        let kind = match unchecked.kind {
            TyKind::U8 => Ok(TyKind::U8),  // TODO: Builtin type variant
            TyKind::Void => Ok(TyKind::Void),
            TyKind::Alias(name) => {
                self.env
                    .get_struct(&name)
                    .map(|_| TyKind::Alias(name.clone()))
                    .ok_or(SE::new(span.clone(), SEK::Undefined(name)))
            },
            TyKind::Ptr(inner) => {
                self.check_ty(*inner)
                    .map(|ty| TyKind::Ptr(Box::new(ty)))
            },
            TyKind::Func(func) => {
                self.check_func_ty(*func)
                    .map(|func| TyKind::Func(Box::new(func)))
            },
            TyKind::Paren(inner) => {
                self.check_ty(*inner)
                    .map(|ty| TyKind::Paren(Box::new(ty)))
            },
        };

        kind.map(|kind| Ty::new(span, kind))
    }

    pub fn check_tys(&self, unchecked: Vec<Ty>) -> CheckResult<Vec<Ty<Checked>>> {
        unchecked
            .into_iter()
            .map(|ty| self.check_ty(ty))
            .collect()
    }

    pub fn check_func_ty(&self, unchecked: FuncTy) -> CheckResult<FuncTy<Checked>> {
        self.check_tys(unchecked.params)
            .join(self.check_ty(unchecked.return_ty))
            .map(|(params, return_ty)| FuncTy::new(params, return_ty))
    }

    pub fn check_field(&mut self, unchecked: Field) -> CheckResult<Field<Checked>> {
        let name = unchecked.name;
        
        self.check_ty(unchecked.ty)
            .and_then(|ty| self.env.declare_binding(name.clone(), ty.clone()).and(Ok(ty)))
            .map(|dt| Field::new(name, dt))
    }

    pub fn check_fields(&mut self, unchecked: Vec<Field>) -> CheckResult<Vec<Field<Checked>>> {
        unchecked
            .into_iter()
            .map(|ty| self.check_field(ty))
            .collect()
    }
}

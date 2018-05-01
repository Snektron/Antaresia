use std::collections::HashMap;
use utility::Scoped;
use check::{CheckResult, SemanticError as SE, SemanticErrorKind as SEK};
use check::Checked;
use ast::ty::{Ty, TyKind, Field};
use ast::{Name, Program};
use ast::{Stmt, Stmts, StmtKind};
use ast::{Expr, ExprKind};
use parser::Span;
use utility::JoinExt;

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

pub struct Checker<'s> {
    scope: Scoped<'s, Frame>
}

impl<'s> Checker<'s> {
    pub fn new() -> Self {
        Checker {
            scope: Scoped::new(Frame::new())
        }
    }

    fn enter<'a>(&'a self) -> Checker<'a> {
        Checker {
            scope: self.scope.enter_with(Frame::new())
        }
    }

    fn declare_binding(&mut self, name: Name, ty: Ty<Checked>) -> CheckResult<()> {
        let span = ty.span.clone();

        self.scope.bindings
            .insert(name.clone(), ty)
            .map_or(Ok(()), |ref existing| {
                Err(SE::new(span, SEK::Redefinition(existing.span.clone(), name)))
            })
    }

    fn declare_struct(&mut self, alias: Name, target: Struct) -> CheckResult<()> {
        let span = target.0.clone();

        self.scope.aliases
            .insert(alias.clone(), target)
            .map_or(Ok(()), |ref existing| {
                Err(SE::new(span, SEK::Redefinition(existing.0.clone(), alias)))
            })
    }

    fn get_binding(&self, name: &Name) -> Option<&Ty<Checked>> {
        self.scope.find(|frame| frame.bindings.get(name))
    }

    fn get_struct(&self, name: &Name) -> Option<&Vec<Field<Checked>>> {
        self.scope
            .find(|frame| frame.aliases.get(name))
            .map(|&(_, ref fields)| fields)
    }

    pub fn forward_declare(&mut self, unchecked: &Stmts) -> CheckResult<()> {
        for stmt in unchecked {
            match stmt.kind {
                StmtKind::FuncDecl(ref name, ref rty, ref params, _) => {
                    {
                        let mut scope = self.enter();

                        params
                            .into_iter()
                            .map(|field| &field.ty)
                            .cloned()
                            .map(|ty| scope.check_ty(ty))
                            .collect::<CheckResult<_>>()
                            .join(scope.check_ty(*rty.clone()))
                            .map(|(params, rty)| Ty::new(stmt.span.clone(), TyKind::Func(Box::new(rty), params)))
                    }.and_then(|ty| self.declare_binding(name.clone(), ty))?;
                },
                StmtKind::StructDecl(ref name, ref fields) => {
                    self.check_fields(fields.to_vec())
                        .and_then(|fields| self.declare_struct(name.clone(), (stmt.span.clone(), fields)))?;
                },
                _ => {}
            }
        }

        Ok(())
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
            _ => {
                Err(SE::new(span.clone(), SEK::IllegalStatement))
            }
        };

        checked.map(|(kind, result)| Expr::new(span, kind, result))
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
                self.get_struct(&name)
                    .map(|_| TyKind::Alias(name.clone()))
                    .ok_or(SE::new(span.clone(), SEK::Undefined(name)))
            },
            TyKind::Ptr(inner) => {
                self.check_ty(*inner)
                    .map(|ty| TyKind::Ptr(Box::new(ty)))
            },
            TyKind::Func(rty, params) => {
                params.into_iter()
                    .map(|par| self.check_ty(par))
                    .collect::<CheckResult<_>>()
                    .join(self.check_ty(*rty))
                    .map(|(params, rty)| TyKind::Func(Box::new(rty), params))
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

    pub fn check_field(&mut self, unchecked: Field) -> CheckResult<Field<Checked>> {
        let name = unchecked.name;
        
        self.check_ty(unchecked.ty)
            .and_then(|ty| self.declare_binding(name.clone(), ty.clone()).and(Ok(ty)))
            .map(|dt| Field::new(name, dt))
    }

    pub fn check_fields(&mut self, unchecked: Vec<Field>) -> CheckResult<Vec<Field<Checked>>> {
        unchecked
            .into_iter()
            .map(|ty| self.check_field(ty))
            .collect()
    }
}

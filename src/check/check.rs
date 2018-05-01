use check::{CheckResult, SemanticError, SemanticErrorKind, Context};
use check::{Unchecked, Checked};
use ast::{Program, Stmt, StmtKind, Expr, ExprKind, UnOpKind};
use ast::ty::{Ty, TyKind, Field};

fn forward_declare<'s>(stmts: &Vec<Stmt>, ctx: &mut Context<'s>) -> CheckResult<()> {
    for stmt in stmts {
        match stmt.kind {
            StmtKind::FuncDecl(ref name, ref rty, ref params, _) => {
                let func = {
                    let mut ctx = &mut ctx.enter();

                    let params: CheckResult<_> = params
                        .into_iter()
                        .map(|field| &field.datatype)
                        .cloned()
                        .map(|dt| dt.check(&mut ctx))
                        .collect();

                    let rty = rty
                        .clone()
                        .check(&mut ctx)?;

                    let kind = TyKind::Func(Box::new(rty), params?);
                    let span = stmt.span.clone();
                    Ty::new(span, kind)
                };
                ctx.declare_binding(name, func)?;
            },
            StmtKind::StructDecl(ref name, ref fields) => {
                let fields = check_fields(fields.to_vec(), &mut ctx.enter())?;
                let span = stmt.span.clone();
                ctx.declare_struct(name, span, fields)?;
            },
            _ => {}
        }
    }

    Ok(())
}

fn check_stmts<'s>(unchecked: Vec<Stmt>, ctx: &mut Context<'s>) -> CheckResult<Vec<Stmt<Checked>>> {
    forward_declare(&unchecked, ctx)?;

    unchecked
        .into_iter()
        .map(|stmt| stmt.check(ctx))
        .collect()
}

fn check_fields<'s>(unchecked: Vec<Field>, ctx: &mut Context<'s>) -> CheckResult<Vec<Field<Checked>>> {
    unchecked
        .into_iter()
        .map(|field| field.check(ctx))
        .collect()
}

fn check_types<'s>(unchecked: Vec<Ty>, ctx: &mut Context<'s>) -> CheckResult<Vec<Ty<Checked>>> {
    unchecked
        .into_iter()
        .map(|dt| dt.check(ctx))
        .collect()
}

fn check_exprs<'s>(unchecked: Vec<Expr>, ctx: &mut Context<'s>) -> CheckResult<Vec<Expr<Checked>>> {
    unchecked
        .into_iter()
        .map(|expr| expr.check(ctx))
        .collect()
}

impl Program<Unchecked> {
    pub fn check<'s>(self, ctx: &mut Context<'s>) -> CheckResult<Program<Checked>> {
        forward_declare(&self.stmts, ctx)?;

        let checked = self.stmts
            .into_iter()
            .map(|stmt| {
                if stmt.is_global_legal() {
                    stmt.check(ctx)
                } else {
                    Err(SemanticError::new(stmt.span, SemanticErrorKind::IllegalStatement))
                }
            })
            .collect::<CheckResult<_>>();

        Ok(Program::new(self.span, checked?))
    }
}

impl Stmt<Unchecked> {
    pub fn check<'s>(self, ctx: &mut Context<'s>) -> CheckResult<Stmt<Checked>> {
        let span = self.span;
        let kind = match self.kind {
            StmtKind::Compound(stmts) => {
                check_stmts(stmts, &mut ctx.enter())
                    .map(|stmts| StmtKind::Compound(stmts))
            },
            StmtKind::Return(expr) => {
                let expr = expr.check(ctx)?;
                let expected = ctx.lookup_return_type().expect("Return statements cannot appear in global scope");
                if &expr.info == expected {
                    Ok(StmtKind::Return(Box::new(expr)))
                } else {
                    let kind = SemanticErrorKind::InvalidReturnType(expected.clone(), expr.info);
                    Err(SemanticError::new(span.clone(), kind))
                }
            },
            StmtKind::FuncDecl(name, rty, params, body) => {
                // name & return type already declared while forward declaring.
                let mut ctx = ctx.enter();
                let params = check_fields(params, &mut ctx)?;
                let rty = rty.check(&mut ctx)?;
                let mut ctx = ctx.enter_func(rty.clone());
                let body = body.check(&mut ctx)?;
                Ok(StmtKind::FuncDecl(name, Box::new(rty), params, Box::new(body)))
            },
            _ => Err(SemanticError::new(span.clone(), SemanticErrorKind::Undefined("WIP-stmt".into())))
        };

        kind.map(|kind| Stmt::new(span, kind))
    }
}

impl Expr<Unchecked> {
    pub fn check<'s>(self, ctx: &mut Context<'s>) -> CheckResult<Expr<Checked>> {
        let span = self.span;
        let kind = match self.kind {
            ExprKind::Binary(op, lhs, rhs) => {
                let lhs = lhs.check(ctx)?;
                let rhs = rhs.check(ctx)?;

                // TODO: implicit casting
                if lhs.info == rhs.info {
                    let result = lhs.info.clone();
                    let kind = ExprKind::Binary(op, Box::new(lhs), Box::new(rhs));
                    Ok((kind, result))
                } else {
                    let kind = SemanticErrorKind::InvalidBinary(op, lhs.info, rhs.info);
                    Err(SemanticError::new(span.clone(), kind))
                }
            },
            ExprKind::Unary(op, lhs) => {
                let lhs = lhs.check(ctx)?;

                match op {
                    UnOpKind::Deref => {
                        let info = lhs.info.clone();
                        let result = lhs.info
                            .clone()
                            .dereference()
                            .ok_or_else(|| {
                                let kind = SemanticErrorKind::InvalidUnary(UnOpKind::Deref, lhs.info.clone());
                                SemanticError::new(span.clone(), kind)
                            })?;

                        let kind = ExprKind::Unary(UnOpKind::Deref, Box::new(lhs));
                        Ok((kind, result))
                    },
                    UnOpKind::Ref => {
                        let actual = lhs.info.clone().reference();
                        Ok((ExprKind::Unary(UnOpKind::Ref, Box::new(lhs)), actual))
                    },
                    _ => {
                        Err(SemanticError::new(span.clone(), SemanticErrorKind::Undefined("WIP-expr".into())))
                    }
                }
            },
            // ExprKind::Call(callee, args) => {
            //     let callee = callee.check(ctx)?;
            //     let args = check_exprs(args)?;

            // },
            ExprKind::Literal(lit) => {
                let ty = lit.ty();
                Ok((ExprKind::Literal(lit), ty))
            },
            ExprKind::Name(name) => {
                ctx.lookup_binding(&name)
                    .ok_or_else(|| {
                        let kind = SemanticErrorKind::Undefined(name.clone());
                        SemanticError::new(span.clone(), kind)
                    })
                    .map(|ty| (ExprKind::Name(name), ty.clone()))
            },
            // ExprKind::Decl(field, None) => {
            //     let field = field.check(ctx)?;
            //     Ok((ExprKind::Decl(field, None), )
            // },
            // ExprKind::Decl(field, Some(value)) => {
            //     let field = field.check(ctx)?;
            //     let value = value.check(ctx)?;
            //     Ok(ExprKind::Decl(field, Some(Box::new(value))))
            // },
            _ => Err(SemanticError::new(span.clone(), SemanticErrorKind::Undefined("WIP-expr".into())))
        };

        kind.map(|(kind, result)| Expr::new(span, kind, result))
    }
}

impl Ty<Unchecked> {
    pub fn check<'s>(self, ctx: &mut Context<'s>) -> CheckResult<Ty<Checked>> {
        let span = self.span;
        let kind = match self.kind {
            TyKind::U8 => Ok(TyKind::U8),
            TyKind::Void => Ok(TyKind::Void),
            TyKind::Alias(name) => {
                if ctx.lookup_struct(&name).is_some() {
                    Ok(TyKind::Alias(name))
                } else {
                    let kind = SemanticErrorKind::Undefined(name);
                    Err(SemanticError::new(span.clone(), kind))
                }
            },
            TyKind::Ptr(inner) => Ok(TyKind::Ptr(Box::new(inner.check(ctx)?))),
            TyKind::Func(rty, params) => {
                let params = check_types(params, ctx)?;
                let rty = rty.check(ctx)?;
                Ok(TyKind::Func(Box::new(rty), params))
            },
            TyKind::Paren(inner) => Ok(TyKind::Paren(Box::new(inner.check(ctx)?))),
        };

        kind.map(|kind| Ty::new(span, kind))
    }
}

impl Field<Unchecked> {
    pub fn check<'s>(self, ctx: &mut Context<'s>) -> CheckResult<Field<Checked>> {
    let name = self.name;
    self.ty
        .check(ctx)
        .and_then(|dt| {
            ctx.declare_binding(&name, dt.clone())?;
            Ok(dt)
        })
        .map(|dt| Field::new(name, dt))
    }
}

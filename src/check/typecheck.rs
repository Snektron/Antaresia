use std::default::Default;
use utility::{JoinExt, TestExt};
use check::{Checked, CheckResult};
use check::environment::{Environment, Binding};
use ast::{Name, Program, Stmt, Stmts, StmtKind, FuncDecl, TypeDecl};
use ast::expr::{Expr, ExprKind, BinOpKind, UnOpKind, Literal};
use ast::ty::{Ty, TyKind, Field, FuncTy, StructTy};
use parser::Span;

type StmtResult = CheckResult<StmtKind<Checked>>;
type ExprResult = CheckResult<(Ty<Checked>, ExprKind<Checked>)>;

lazy_static! {
    static ref BOOL: Ty<Checked> = Ty::new(Default::default(), TyKind::U8);
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

    pub fn forward_declare_types(&mut self, unchecked: &Stmts) -> CheckResult<()> {
        unchecked
            .iter()
            .filter_map(|stmt| match stmt.kind {
                StmtKind::TypeDecl(ref decl) => Some(decl),
                _ => None
            })
            .for_each(|decl| {
                
            });

        Ok(())
    }

    pub fn forward_declare(&mut self, unchecked: &Stmts) -> CheckResult<()> {
        for stmt in unchecked {
            match stmt.kind {
                StmtKind::FuncDecl(ref decl) => {
                    let ty = self.check_func_ty(decl.ty())?;
                    let ty = Ty::new(stmt.span.clone(), TyKind::Func(Box::new(ty)));

                    let binding = Binding {
                        span: stmt.span.clone(),
                        name: decl.name.clone(),
                        ty: ty
                    };

                    self.env.declare_binding(binding)?;
                },
                StmtKind::TypeDecl(ref decl) => {
                    let ty = self.check_ty(decl.ty.clone())?;

                    let binding = Binding {
                        span: stmt.span.clone(),
                        name: decl.name.clone(),
                        ty: ty
                    };

                    self.env.declare_alias(binding)?;
                },
                _ => {}
            }
        }

        Ok(())
    }

    pub fn implicit_convert(&self, actual: Expr<Checked>, expected: &Ty<Checked>) -> CheckResult<Expr<Checked>> {
        if &actual.info == expected {
            return Ok(actual)
        }

        err!(actual.span, "Cannot convert from type '{}' to '{}'", actual.info, expected)
    }

    pub fn check_program(&mut self, unchecked: Program) -> CheckResult<Program<Checked>> {
        self.forward_declare(&unchecked.stmts)?;

        let checked = unchecked.stmts
            .into_iter()
            .map(|stmt| {
                if stmt.is_global_legal() {
                    self.check_stmt(stmt)
                } else {
                    err!(stmt.span, "Illegal statement")
                }
            })
            .collect::<CheckResult<_>>()?;

        Ok(Program::new(unchecked.span, checked))
    }

    pub fn check_stmt(&mut self, unchecked: Stmt) -> CheckResult<Stmt<Checked>> {
        let span = unchecked.span;

        let checked = match unchecked.kind {
            StmtKind::Compound(stmts) => self.check_compound(stmts),
            StmtKind::If(cond, csq, None) => self.check_if(*cond, *csq),
            StmtKind::If(cond, csq, Some(alt)) => self.check_if_else(*cond, *csq, *alt),
            StmtKind::While(cond, body) => self.check_while(*cond, *body),
            StmtKind::Return(..) => unimplemented!(),
            StmtKind::Expr(expr) => self.check_expr_stmt(*expr),
            StmtKind::FuncDecl(func) => self.check_func_decl(*func),
            StmtKind::TypeDecl(decl) => self.check_type_decl(*decl)
        };

        checked.map(|kind| Stmt::new(span, kind))
    }

    pub fn check_stmts(&mut self, unchecked: Stmts) -> CheckResult<Stmts<Checked>> {
        self.forward_declare(&unchecked)?;

        unchecked
            .into_iter()
            .map(|stmt| self.check_stmt(stmt))
            .collect()
    }
    
    fn check_compound(&mut self, stmts: Stmts) -> StmtResult {
        self.enter()
            .check_stmts(stmts)
            .map(|stmts| StmtKind::Compound(stmts))
    }

    fn check_if(&mut self, cond: Expr, csq: Stmt) -> StmtResult {
        let cond = self.check_expr(cond)?;
        let cond = self.implicit_convert(cond, &BOOL)?;

        let csq = self.enter().check_stmt(csq)?;

        Ok(StmtKind::If(Box::new(cond), Box::new(csq), None))
    }

    fn check_if_else(&mut self, cond: Expr, csq: Stmt, alt: Stmt) -> StmtResult {
        let cond = self.check_expr(cond)?;
        let cond = self.implicit_convert(cond, &BOOL)?;

        let csq = self.enter().check_stmt(csq)?;
        let alt = self.enter().check_stmt(alt)?;

        Ok(StmtKind::If(Box::new(cond), Box::new(csq), Some(Box::new(alt))))
    }

    fn check_while(&mut self, cond: Expr, body: Stmt) -> StmtResult {
        let cond = self.check_expr(cond)?;
        let cond = self.implicit_convert(cond, &BOOL)?;

        let body = self.enter().check_stmt(body)?;

        Ok(StmtKind::While(Box::new(cond), Box::new(body)))
    }

    fn check_expr_stmt(&mut self, expr: Expr) -> StmtResult {
        self.check_expr(expr)
            .map(|expr| StmtKind::Expr(Box::new(expr)))
    }

    fn check_func_decl(&mut self, decl: FuncDecl) -> StmtResult {
        // function signature already forward declared.
        let mut checker = self.enter();
        let params = checker.check_fields(decl.params)?;
        let return_ty = checker.check_ty(decl.return_ty)?;
        let body = checker.enter().check_stmt(decl.body)?;

        let decl = FuncDecl::new(decl.name, params, return_ty, body);
        Ok(StmtKind::FuncDecl(Box::new(decl)))
    }

    fn check_type_decl(&mut self, decl: TypeDecl) -> StmtResult {
        // type itself already forward declared.
        let name = decl.name;
        let ty = decl.ty;

        self.check_ty(ty)
            .map(|ty| StmtKind::TypeDecl(Box::new(TypeDecl::new(name, ty))))
    }

    pub fn check_expr(&mut self, unchecked: Expr) -> CheckResult<Expr<Checked>> {
        let span = unchecked.span;

        let checked = match unchecked.kind {
            ExprKind::Binary(op, lhs, rhs) => self.check_binary(span.clone(), op, *lhs, *rhs),
            ExprKind::Unary(op, arg) => self.check_unary(span.clone(), op, *arg),
            ExprKind::Call(callee, args) => self.check_call(*callee, args),
            ExprKind::Cast(..) => unimplemented!(),
            ExprKind::ImplicitCast(..) => unimplemented!(),
            ExprKind::Literal(lit) => self.check_literal(span.clone(), lit),
            ExprKind::Name(name) => self.check_name(span.clone(), name),
            ExprKind::Decl(field, default) => self.check_decl(span.clone(), field, default)
        };

        checked.map(|(result_ty, kind)| Expr::new(span, kind, result_ty))
    }

    pub fn check_exprs(&mut self, unchecked: Vec<Expr>) -> CheckResult<Vec<Expr<Checked>>> {
        unchecked
            .into_iter()
            .map(|expr| self.check_expr(expr))
            .collect()
    }

    fn check_binary(&mut self, span: Span, opkind: BinOpKind, lhs: Expr, rhs: Expr) -> ExprResult {
        unimplemented!();
    }

    fn check_unary(&mut self, span: Span, opkind: UnOpKind, arg: Expr) -> ExprResult {
        unimplemented!();
    }

    fn check_call(&mut self, callee: Expr, args: Vec<Expr>) -> ExprResult {
        let callee = self.check_expr(callee)?;

        let func = if let TyKind::Func(ref func) = callee.info.kind {
            let return_ty = func.return_ty.clone();

            let args = self
                .check_exprs(args)?
                .into_iter()
                .zip(func.params.iter())
                .map(|(arg, param_ty)| self.implicit_convert(arg, param_ty))
                .collect::<CheckResult<_>>()?;
            Ok((return_ty, args))
        } else {
            err!(callee.span.clone(), "'{}' is not a function", callee.info)
        };

        func.map(|(return_ty, args)| (return_ty, ExprKind::Call(Box::new(callee), args)))
    }

    fn check_literal(&self, span: Span, lit: Literal) -> ExprResult {
        let ty = match &lit {
            Literal::Integer(_) => Ty::new(span, TyKind::U8)
        };

        Ok((ty, ExprKind::Literal(lit)))
    }

    fn check_name(&self, span: Span, name: Name) -> ExprResult {
        self.env
            .binding(&name)
            .map(|binding| &binding.ty)
            .cloned()
            .ok_or(fmt_err!(span.clone(), "Undefined variable '{}'", name))
            .join(Ok(ExprKind::Name(name)))
    }

    fn check_decl(&mut self, span: Span, field: Field, default: Option<Box<Expr>>) -> ExprResult {
        let field = self.check_field(field)?;

        let expr = match default {
            None => Ok(None),
            Some(expr) => {
                self.check_expr(*expr)
                    .and_then(|expr| self.implicit_convert(expr, &field.ty))
                    .map(|expr| Some(Box::new(expr)))
            }
        }?;

        let ty = if expr.is_some() {
            field.ty.clone()
        } else {
            Ty::void(span.clone())
        };

        let kind = ExprKind::Decl(field, expr);
        Ok((ty, kind))
    }

    pub fn check_ty(&self, unchecked: Ty) -> CheckResult<Ty<Checked>> {
        let span = unchecked.span;

        let kind = match unchecked.kind {
            TyKind::U8 => Ok(TyKind::U8),  // TODO: Builtin/integral type variant?
            TyKind::Void => Ok(TyKind::Void),
            TyKind::Alias(name) => {
                self.env
                    .alias(&name)
                    .map(|_| TyKind::Alias(name.clone()))
                    .ok_or(fmt_err!(span.clone(), "Undefined type '{}'", name))
            },
            TyKind::Ptr(inner) => {
                self.check_ty(*inner)
                    .map(|ty| TyKind::Ptr(Box::new(ty)))
            },
            TyKind::Func(func) => {
                self.check_func_ty(*func)
                    .map(|func| TyKind::Func(Box::new(func)))
            },
            TyKind::Struct(strukt) => {
                let fields = self
                    .enter()
                    .check_fields(strukt.fields)?;
                Ok(TyKind::Struct(Box::new(StructTy::new(fields))))
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

    pub fn check_param_tys(&self, unchecked: Vec<Ty>) -> CheckResult<Vec<Ty<Checked>>> {
        unchecked
            .into_iter()
            .map(|ty| self.check_ty(ty))
            .map(|ty| {
                ty.test_or(|ty| !ty.is_void(), |ty| {
                    fmt_err!(ty.span.clone(), "Parameter cannot have type '{}'", ty)
                })
            })
            .collect()
    }

    pub fn check_func_ty(&self, unchecked: FuncTy) -> CheckResult<FuncTy<Checked>> {
        self.check_param_tys(unchecked.params)
            .join(self.check_ty(unchecked.return_ty))
            .map(|(params, return_ty)| FuncTy::new(params, return_ty))
    }

    pub fn check_field(&mut self, unchecked: Field) -> CheckResult<Field<Checked>> {
        let span = unchecked.span;
        let name = unchecked.name;
        
        self.check_ty(unchecked.ty)
            .test_or(|ty| !ty.is_void(), |ty| {
                fmt_err!(ty.span.clone(), "Field cannot have type '{}'", ty)
            })
            .and_then(|ty| {
                let binding = Binding {
                    span: span.clone(),
                    name: name.clone(),
                    ty: ty.clone()
                };

                self.env
                    .declare_binding(binding)
                    .and(Ok(ty))
            })
            .map(|dt| Field::new(span, name, dt))
    }

    pub fn check_fields(&mut self, unchecked: Vec<Field>) -> CheckResult<Vec<Field<Checked>>> {
        unchecked
            .into_iter()
            .map(|ty| self.check_field(ty))
            .collect()
    }
}

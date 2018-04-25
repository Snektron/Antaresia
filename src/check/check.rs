use std::default::Default;
use check::{CheckResult, SemanticError, SemanticErrorKind, Context};
use check::{Unchecked, Checked};
use ast::{Program, Stmt, StmtKind, Expr};
use ast::{DataType, DataTypeKind, Field};

// forward insert type and function definitions
fn forward_declare<'s>(stmts: &Vec<Stmt>, ctx: &mut Context<'s>) -> CheckResult<()> {
    for stmt in stmts {
        match stmt.kind {
            StmtKind::FuncDecl(ref name, ref rtype, ref params, _) => {
                let func = {
                    let mut ctx = &mut ctx.enter();
                    let params: CheckResult<Vec<DataType<Checked>>> = params.into_iter().map(|field| &field.datatype).cloned().map(|dt| dt.check(&mut ctx)).collect();
                    let params = params?;
                    let rtype = Box::new(rtype.clone().check(&mut ctx)?);
                    DataType::new(stmt.span.clone(), DataTypeKind::Func(rtype, params))
                };
                ctx.declare_binding(name, func)?;
            },
            StmtKind::StructDecl(ref name, ref fields) => {
                let fields = {
                     let mut ctx = &mut ctx.enter();
                     check_fields(fields.to_vec(), &mut ctx)?
                };
                ctx.declare_struct(name, stmt.span.clone(), fields)?;
            },
            _ => {}
        }
    }

    Ok(())
}

fn check_stmts<'s>(unchecked: Vec<Stmt<Unchecked>>, ctx: &mut Context<'s>) -> CheckResult<Vec<Stmt<Checked>>> {
    forward_declare(&unchecked, ctx)?;

    let mut checked = Vec::new();

    for stmt in unchecked.into_iter() {
        checked.push(stmt.check(ctx)?);
    }

    Ok(checked)
}

fn check_fields<'s>(unchecked: Vec<Field<Unchecked>>, ctx: &mut Context<'s>) -> CheckResult<Vec<Field<Checked>>> {
    let mut checked = Vec::new();

    for field in unchecked {
        let datatype = field.datatype.check(ctx)?;
        ctx.declare_binding(&field.name, datatype.clone())?;
        checked.push(Field::new(field.name, datatype));
    }

    Ok(checked)
}

impl Program<Unchecked> {
    pub fn check<'s>(self, ctx: &mut Context<'s>) -> CheckResult<Program<Checked>> {
        forward_declare(&self.stmts, ctx)?;

        let mut checked = Vec::new();

        for stmt in self.stmts.into_iter() {
            if stmt.is_global_legal() {
                checked.push(stmt.check(ctx)?);
            } else {
                return Err(SemanticError::new(stmt.span, SemanticErrorKind::IllegalStatement));
            }
        }

        Ok(Program::new(self.span, checked))
    }
}

impl Stmt<Unchecked> {
    pub fn check<'s>(self, ctx: &mut Context<'s>) -> CheckResult<Stmt<Checked>> {
        match self.kind {
            StmtKind::Compound(stmts) => {
                let stmts = check_stmts(stmts, &mut ctx.enter())?;
                Ok(Stmt::new(self.span, StmtKind::Compound(stmts)))
            },
            StmtKind::FuncDecl(name, rtype, params, body) => {
                // name & return type already declared while forward declaring.
                let mut ctx = ctx.enter();
                let params = check_fields(params, &mut ctx)?;
                let rtype = Box::new(rtype.check(&mut ctx)?);
                let body = Box::new(body.check(&mut ctx)?);
                Ok(Stmt::new(self.span, StmtKind::FuncDecl(name, rtype, params, body)))
            },
            _ => Err(SemanticError::new(Default::default(), SemanticErrorKind::OutOfScope("WIP".into())))
        }
    }
}

impl Expr<Unchecked> {
    pub fn check<'s>(self, ctx: &mut Context<'s>) -> CheckResult<Expr<Checked>> {
        Err(SemanticError::new(Default::default(), SemanticErrorKind::OutOfScope("WIP".into())))
    }
}

impl DataType<Unchecked> {
    pub fn check<'s>(self, ctx: &mut Context<'s>) -> CheckResult<DataType<Checked>> {
        Err(SemanticError::new(Default::default(), SemanticErrorKind::OutOfScope("WIP".into())))
    }
}
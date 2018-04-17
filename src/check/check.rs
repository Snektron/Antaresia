use check::{SemanticError, Context};
use check::{Unchecked, Checked};
use ast::{Program, Stmt, StmtKind, Expr};
use ast::{DataType, DataTypeKind, Field};

// forward insert type and function definitions
fn forward_declare<'s>(stmts: &Vec<Stmt>, ctx: &mut Context<'s>) -> Result<(), SemanticError> {
    for stmt in stmts {
        match stmt.kind {
            StmtKind::FuncDecl(ref name, ref rtype, ref params, _) => {
                let params = params.into_iter().map(|field| &field.datatype).cloned().collect();
                let func = DataType::new(DataTypeKind::Func(rtype.clone(), params));
                ctx.declare_binding(name, func)?;
            },
            StmtKind::StructDecl(ref name, ref fields) => {
                ctx.declare_struct(name, fields.to_vec())?;
            },
            _ => {}
        }
    }

    Ok(())
}

fn check_stmts<'s>(unchecked: Vec<Stmt<Unchecked>>, ctx: &mut Context<'s>) -> Result<Vec<Stmt<Checked>>, SemanticError> {
    forward_declare(&unchecked, ctx)?;

    let mut checked = Vec::new();

    for stmt in unchecked.into_iter() {
        checked.push(stmt.check(ctx)?);
    }

    Ok(checked)
}

fn check_fields<'s>(unchecked: Vec<Field<Unchecked>>, ctx: &mut Context<'s>) -> Result<Vec<Field<Checked>>, SemanticError> {
    Err(SemanticError::Redefinition("WIP".into()))
}

fn declare_fields<'s>(fields: &Vec<Field>, ctx: &mut Context<'s>) -> Result<(), SemanticError> {
    for field in fields {
        ctx.declare_binding(&field.name, field.datatype.clone())?;
    }

    Ok(())
}

impl Program<Unchecked> {
    pub fn check<'s>(self, ctx: &mut Context<'s>) -> Result<Program<Checked>, SemanticError> {
        Ok(Program::new(check_stmts(self.stmts, ctx)?))
    }
}

impl Stmt<Unchecked> {
    pub fn check<'s>(self, ctx: &mut Context<'s>) -> Result<Stmt<Checked>, SemanticError> {
        match self.kind {
            StmtKind::Compound(stmts) => {
                let stmts = check_stmts(stmts, &mut ctx.enter())?;
                Ok(Stmt::new(StmtKind::Compound(stmts)))
            },
            StmtKind::FuncDecl(name, rtype, params, body) => {
                // name & return type already declared while forward declaring.
                let mut ctx = ctx.enter();
                declare_fields(&params, &mut ctx)?;
                let params = check_fields(params, &mut ctx)?;
                let rtype = Box::new(rtype.check(&mut ctx)?);
                let body = Box::new(body.check(&mut ctx)?);
                Ok(Stmt::new(StmtKind::FuncDecl(name, rtype, params, body)))
            },
            _ => Err(SemanticError::Redefinition("WIP".into()))
        }
    }
}

impl Expr<Unchecked> {
    pub fn check<'s>(self, ctx: &mut Context<'s>) -> Result<Expr<Checked>, SemanticError> {
        Err(SemanticError::Redefinition("WIP".into()))
    }
}

impl DataType<Unchecked> {
    pub fn check<'s>(self, ctx: &mut Context<'s>) -> Result<DataType<Checked>, SemanticError> {
        Err(SemanticError::Redefinition("WIP".into()))
    }
}
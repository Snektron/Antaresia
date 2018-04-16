use check::{SemanticError, Context};
use check::{Unchecked, Checked};
use ast::{Program, Stmt, StmtKind, Expr};

fn check_stmts<'s>(unchecked: Vec<Stmt<Unchecked>>, ctx: &mut Context<'s>) -> Result<Vec<Stmt<Checked>>, SemanticError> {
    ctx.forward_declare(&unchecked)?;

    let mut checked = Vec::new();

    for stmt in unchecked.into_iter() {
        checked.push(stmt.check(ctx)?);
    }

    Ok(checked)
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
            _ => Err(SemanticError::Redefinition("WIP".into()))
        }
    }
}

impl Expr<Unchecked> {
    pub fn check<'s>(self, ctx: &mut Context<'s>) -> Result<Expr<Checked>, SemanticError> {
        Err(SemanticError::Redefinition("WIP".into()))
    }
}

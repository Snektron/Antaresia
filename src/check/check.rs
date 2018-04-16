use check::{Check, SemanticError, Frame, Context};
use check::{UncheckedExprInfo, CheckedExprInfo};
use check::{UncheckedStmtInfo, CheckedStmtInfo};
use ast::{Program, Stmt, Expr};

impl Check for Program<UncheckedStmtInfo> {
    type Target = Program<CheckedStmtInfo>;

    fn check<'s>(self, ctx: &mut Context<'s>) -> Result<Self::Target, SemanticError> {
        ctx.forward_insert(&self.stmts)?;

        let mut stmts = Vec::new();

        for stmt in self.stmts.into_iter() {
            stmts.push(stmt.check(ctx)?);
        }

        Ok(Program {
            stmts
        })
    }
}

impl Check for Stmt<UncheckedStmtInfo> {
    type Target = Stmt<CheckedStmtInfo>;

    fn check<'s>(self, ctx: &mut Context<'s>) -> Result<Self::Target, SemanticError> {
        Err(SemanticError::Redefinition("test".into()))
    }
}

impl Check for Expr<UncheckedExprInfo> {
    type Target = Expr<CheckedExprInfo>;

    fn check<'s>(self, ctx: &mut Context<'s>) -> Result<Self::Target, SemanticError> {
        Err(SemanticError::Redefinition("test".into()))
    }
}

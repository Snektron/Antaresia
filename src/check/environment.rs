use std::collections::HashMap;
use utility::Scoped;
use check::{CheckResult, SemanticError as SE, SemanticErrorKind as SEK};
use check::Checked;
use ast::ty::{Ty, TyKind, Field, FuncTy};
use ast::Name;
use parser::Span;

pub struct Frame {
    bindings: HashMap<Name, Ty<Checked>>,
    types: HashMap<Name, Ty<Checked>>,
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            bindings: HashMap::new(),
            types: HashMap::new(),
        }
    }
}

pub struct Environment<'s> {
    scope: Scoped<'s, Frame>
}

impl<'s> Environment<'s> {
    pub fn new() -> Self {
        Self {
            scope: Scoped::new(Frame::new())
        }
    }

    pub fn enter<'a>(&'a self) -> Environment<'a> {
        Environment {
            scope: self.scope.enter_with(Frame::new())
        }
    }

    pub fn declare_binding(&mut self, name: Name, ty: Ty<Checked>) -> CheckResult<()> {
        let span = ty.span.clone();

        self.scope.bindings
            .insert(name.clone(), ty)
            .map_or(Ok(()), |ref existing| {
                Err(SE::new(span, SEK::Redefinition(existing.span.clone(), name)))
            })
    }

    pub fn declare_ty(&mut self, alias: Name, ty: Ty<Checked>) -> CheckResult<()> {
        let span = ty.span.clone();

        self.scope.types
            .insert(alias.clone(), ty)
            .map_or(Ok(()), |ref existing| {
                Err(SE::new(span, SEK::Redefinition(existing.span.clone(), alias)))
            })
    }

    pub fn get_binding(&self, name: &Name) -> Option<&Ty<Checked>> {
        self.scope.find(|frame| frame.bindings.get(name))
    }

    pub fn get_ty(&self, name: &Name) -> Option<&Ty<Checked>> {
        self.scope.find(|frame| frame.types.get(name))
    }
}
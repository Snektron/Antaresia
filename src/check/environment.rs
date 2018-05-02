use std::collections::HashMap;
use utility::Scoped;
use check::{CheckResult, SemanticError as SE, SemanticErrorKind as SEK};
use check::Checked;
use ast::ty::{Ty, TyKind, Field, FuncTy};
use ast::Name;
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

pub struct Environment {
    scope: Scoped<'s, Frame>
}

impl Environment {
    pub fn new() -> Self {
        Self {
            scope: Scoped::new(Frame::new())
        }
    }

    pub fn enter(&self) -> Self {
        Self {
            scope: scope.enter_with(Frame::new())
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

    pub fn declare_struct(&mut self, alias: Name, target: Struct) -> CheckResult<()> {
        let span = target.0.clone();

        self.scope.aliases
            .insert(alias.clone(), target)
            .map_or(Ok(()), |ref existing| {
                Err(SE::new(span, SEK::Redefinition(existing.0.clone(), alias)))
            })
    }

    pub fn get_binding(&self, name: &Name) -> Option<&Ty<Checked>> {
        self.scope.find(|frame| frame.bindings.get(name))
    }

    pub fn get_struct(&self, name: &Name) -> Option<&Vec<Field<Checked>>> {
        self.scope
            .find(|frame| frame.aliases.get(name))
            .map(|&(_, ref fields)| fields)
    }

}
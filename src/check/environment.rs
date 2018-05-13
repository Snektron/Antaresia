use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::cmp::{PartialEq, Eq};
use std::borrow::Borrow;
use utility::scoped::Scoped;
use check::{Checked, CheckResult};
use ast::ty::Ty;
use ast::Name;
use parser::Span;

#[derive(Clone)]
pub struct Binding {
    pub span: Span,
    pub name: Name,
    pub ty: Ty<Checked>
}

impl Hash for Binding {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.name.hash(hasher);
    }
}

impl PartialEq for Binding {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl PartialEq<Name> for Binding {
    fn eq(&self, other: &Name) -> bool {
        &self.name == other
    }
}

impl Eq for Binding {}

impl Borrow<Name> for Binding {
    fn borrow(&self) -> &Name {
        &self.name
    }
}

pub struct Frame {
    bindings: HashSet<Binding>,
    types: HashSet<Binding>
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            bindings: HashSet::new(),
            types: HashSet::new(),
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

    pub fn declare_binding(&mut self, binding: Binding) -> CheckResult<()> {
        if let Some(existing) = self.scope.bindings.get(&binding) {
            return err!(binding.span, "Variable '{}' is already defined at {}", binding.name, existing.span)
        }


        self.scope.bindings.insert(binding);
        Ok(())
    }

    pub fn binding(&self, name: &Name) -> Option<&Binding> {
        self.scope
            .find(|frame| frame.bindings.get(name))
    }

    pub fn declare_alias(&mut self, alias: Binding) -> CheckResult<()> {
        if let Some(existing) = self.scope.types.get(&alias) {
            return err!(alias.span, "Type '{}' is already defined at {}", alias.name, existing.span)
        }

        self.scope.types.insert(alias);
        Ok(())
    }

    pub fn alias(&self, name: &Name) -> Option<&Binding> {
        self.scope
            .find(|frame| frame.types.get(name))
    }
}

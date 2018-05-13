use std::rc::Rc;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::cmp::{PartialEq, Eq};
use std::borrow::Borrow;
use check::{Checked, CheckResult};
use ast::Name;
use ast::ty::Ty;
use parser::Span;

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

pub struct Scope {
    parent: Option<Rc<Scope>>,
    bindings: HashSet<Binding>,
    types: HashSet<Binding>
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            parent: None,
            bindings: HashSet::new(),
            types: HashSet::new()
        }
    }

    pub fn enter(parent: Rc<Scope>) -> Scope {
        Scope {
            parent: Some(parent),
            bindings: HashSet::new(),
            types: HashSet::new()
        }
    }

    pub fn declare_binding(&mut self, binding: Binding) -> CheckResult<()> {
        if let Some(existing) = self.bindings.get(&binding) {
            return err!(binding.span, "Variable '{}' is already defined at {}", binding.name, existing.span)
        }

        self.bindings.insert(binding);
        Ok(())
    }

    pub fn binding(&self, name: &Name) -> Option<&Binding> {
        self.search(|scope| scope.bindings.get(name))
    }

    pub fn declare_alias(&mut self, alias: Binding) -> CheckResult<()> {
        if let Some(existing) = self.types.get(&alias) {
            return err!(alias.span, "Type '{}' is already defined at {}", alias.name, existing.span)
        }

        self.types.insert(alias);
        Ok(())
    }

    pub fn alias(&self, name: &Name) -> Option<&Binding> {
        self.search(|scope| scope.types.get(name))
    }

    pub fn iter<'a>(&'a self) -> Iter<'a> {
        Iter {
            scope: self
        }
    }

    pub fn search<'a, F, R>(&'a self, func: F) -> Option<R>
    where F: Fn(&'a Scope) -> Option<R> {
        self.iter()
            .filter_map(func)
            .next()
    }
}

pub struct Iter<'a> {
    scope: &'a Scope
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Scope;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref parent) = self.scope.parent {
            self.scope = &*parent;
            Some(self.scope)
        } else {
            None
        }
    }
}
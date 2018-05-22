use std::iter::Iterator;
use std::hash::{Hash, Hasher};
use std::cmp::{PartialEq, Eq};
use std::borrow::Borrow;
use std::collections::HashSet;
use ast::Name;
use parser::Span;

pub struct Symbol {
    pub span: Span,
    pub name: Name,
    pub scope: ScopeId
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Symbol) -> bool {
        self.name == other.name
    }
}

impl PartialEq<Name> for Symbol {
    fn eq(&self, other: &Name) -> bool {
        &self.name == other
    }
}

impl Borrow<Name> for Symbol {
    fn borrow(&self) -> &Name {
        &self.name
    }
}

impl Eq for Symbol {}

impl Hash for Symbol {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.name.hash(hasher)
    }
}

pub struct Scope {
    index: ScopeId,
    parent: Option<ScopeId>,
    symbols: HashSet<Symbol>
}

impl Scope {
    fn new(index: ScopeId, parent: Option<ScopeId>) -> Self {
        Scope {
            index,
            parent,
            symbols: HashSet::new()
        }
    }

    pub fn insert(&mut self, span: Span, name: Name) -> bool {
        let sym = Symbol {
            span,
            name,
            scope: self.index
        };

        self.symbols.insert(sym)
    }

    pub fn lookup(&self, name: &Name) -> Option<&Symbol> {
        self.symbols.get(name)
    }
}

pub type ScopeId = usize;

pub struct ScopeTree {
    scopes: Vec<Scope>
}

impl ScopeTree {
    pub fn new() -> Self {
        ScopeTree {
            scopes: Vec::new()
        }
    }

    pub fn add_scope(&mut self, parent: Option<ScopeId>) -> ScopeId {
        let scope = self.scopes.len();
        self.scopes.push(Scope::new(scope, parent));
        scope
    }

    pub fn scope(&self, scope: ScopeId) -> &Scope {
        &self.scopes[scope]
    }

    pub fn scope_mut(&mut self, scope: ScopeId) -> &mut Scope {
        &mut self.scopes[scope]
    }

    pub fn iter<'a>(&'a self, from: ScopeId) -> Iter<'a> {
        Iter {
            scope_tree: self,
            current: Some(from)
        }
    }
}

pub struct Iter<'a> {
    scope_tree: &'a ScopeTree,
    current: Option<ScopeId>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Scope;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current {
            let scope = self.scope_tree.scope(current);
            self.current = scope.parent;
            Some(scope)
        } else {
            None
        }
    }
}


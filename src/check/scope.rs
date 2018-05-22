use std::rc::{Rc, Weak};
use std::hash::{Hash, Hasher};
use std::cmp::{PartialEq, Eq};
use std::borrow::Borrow;
use std::collections::HashSet;
use std::cell::RefCell;
use ast::Name;
use parser::Span;

#[derive(Hash, PartialEq, Eq)]
pub enum SymbolKind {
    Type(Name)
}

pub struct Symbol {
    pub span: Span,
    pub kind: SymbolKind,
    scope: Weak<RefCell<Inner>>
}

impl Symbol {
    pub fn scope(&self) -> Scope {
        self.scope
            .upgrade()
            .map(Scope::from_inner)
            .expect("Symbol has no parent scope")
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Symbol) -> bool {
        self.kind == other.kind
    }
}

impl PartialEq<SymbolKind> for Symbol {
    fn eq(&self, other: &SymbolKind) -> bool {
        &self.kind == other
    }
}

impl Borrow<SymbolKind> for Symbol {
    fn borrow(&self) -> &SymbolKind {
        &self.kind
    }
}

impl Eq for Symbol {}

impl Hash for Symbol {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.kind.hash(hasher)
    }
}

pub struct Scope {
    inner: Rc<RefCell<Inner>>
}

struct Inner {
    parent: Option<Rc<RefCell<Inner>>>,
    symbols: HashSet<Rc<Symbol>>
}

impl Inner {
    fn new() -> Inner {
        Inner {
            parent: None,
            symbols: HashSet::new()
        }
    }

    fn enter(parent: Rc<RefCell<Inner>>) -> Self {
        Inner {
            parent: Some(parent),
            symbols: HashSet::new()
        }
    }
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            inner: Rc::new(RefCell::new(Inner::new()))
        }
    }

    fn from_inner(inner: Rc<RefCell<Inner>>) -> Self {
        Scope {
            inner
        }
    }

    pub fn enter(&self) -> Scope {
        Scope {
            inner: Rc::new(RefCell::new(Inner::enter(self.inner.clone())))
        }
    }

    pub fn insert(&mut self, span: Span, kind: SymbolKind) -> bool {
        let sym = Symbol {
            span,
            kind,
            scope: Rc::downgrade(&self.inner)
        };

        self.inner.borrow_mut().symbols.insert(Rc::new(sym))
    }
}
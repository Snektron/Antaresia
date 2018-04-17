use std::collections::HashMap;
use check::{CheckResult, SemanticError, SemanticErrorKind};
use check::scoped::{Scoped, Iter as ScopeIter};
use ast::{DataType, Field};
use ast::Name;
use parser::Span;

pub struct Frame {
    bindings: HashMap<Name, DataType>,
    structs: HashMap<Name, (Span, Vec<Field>)>
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            bindings: HashMap::new(),
            structs: HashMap::new()
        }
    }
}

pub struct Context<'s> {
    scope: Scoped<'s, Frame>
}

impl<'s> Context<'s> {
    pub fn new() -> Self {
        Context {
            scope: Scoped::new(Frame::new())
        }
    }

    pub fn enter<'a>(&'a self) -> Context<'a> {
        Context {
            scope: self.scope.enter_with(Frame::new())
        }
    }

    pub fn iter<'a>(&'a self) -> ScopeIter<'a, Frame> {
        self.scope.iter()
    }

    pub fn declare_binding(&mut self, name: &Name, dt: DataType) -> CheckResult<()> {
        let span = dt.span.clone();

        match self.scope.item_mut().bindings.insert(name.clone(), dt) {
            Some(ref original) => Err(SemanticError::new(span, SemanticErrorKind::Redefinition(original.span.clone(), name.clone()))),
            None => Ok(())
        }
    }

    pub fn declare_struct(&mut self, name: &Name, span: Span, fields: Vec<Field>) -> CheckResult<()> {
        match self.scope.item_mut().structs.insert(name.clone(), (span.clone(), fields)) {
            Some((origin, _)) => Err(SemanticError::new(span, SemanticErrorKind::Redefinition(origin.clone(), name.clone()))),
            None => Ok(())
        }
    }

    pub fn lookup_binding(&self, name: &Name) -> Option<&DataType> {
        for scope in self.iter() {
            if let Some(dt) = scope.bindings.get(name) {
                return Some(dt);
            }
        }

        None
    }

    pub fn lookup_struct(&self, name: &Name) -> Option<&Vec<Field>> {
        for scope in self.iter() {
            if let Some(fields) = scope.structs.get(name) {
                return Some(&fields.1);
            }
        }

        None
    }
}

use std::collections::HashMap;
use check::SemanticError;
use check::scoped::{Scoped, Iter as ScopeIter};
use ast::{DataType, Field};
use ast::Name;

pub struct Frame {
    bindings: HashMap<Name, DataType>,
    structs: HashMap<Name, Vec<Field>>
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

    pub fn declare_binding(&mut self, name: &Name, dt: DataType) -> Result<(), SemanticError> {
        match self.scope.item_mut().bindings.insert(name.clone(), dt) {
            Some(_) => Err(SemanticError::Redefinition(name.clone())),
            None => Ok(())
        }
    }

    pub fn declare_struct(&mut self, name: &Name, fields: Vec<Field>) -> Result<(), SemanticError> {
        match self.scope.item_mut().structs.insert(name.clone(), fields) {
            Some(_) => Err(SemanticError::Redefinition(name.clone())),
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
            if let Some(dt) = scope.structs.get(name) {
                return Some(dt);
            }
        }

        None
    }
}

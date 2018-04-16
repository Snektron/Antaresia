use std::collections::HashMap;
use check::SemanticError;
use check::scoped::{Scoped, Iter as ScopeIter};
use datatype::{DataType, Field, Struct};
use ast::{Name, Stmt, StmtKind};

pub struct Frame {
    bindings: HashMap<Name, DataType>,
    structs: HashMap<Name, Struct>
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

    pub fn insert_binding(&mut self, name: &Name, dt: DataType) -> Result<(), SemanticError> {
        match self.scope.item_mut().bindings.insert(name.clone(), dt) {
            Some(_) => Err(SemanticError::Redefinition(name.clone())),
            None => Ok(())
        }
    }

    pub fn insert_struct(&mut self, name: &Name, dt: Struct) -> Result<(), SemanticError> {
        match self.scope.item_mut().structs.insert(name.clone(), dt) {
            Some(_) => Err(SemanticError::Redefinition(name.clone())),
            None => Ok(())
        }
    }

    pub fn get_binding(&self, name: &Name) -> Option<&DataType> {
        for scope in self.iter() {
            if let Some(dt) = scope.bindings.get(name) {
                return Some(dt);
            }
        }

        None
    }

    pub fn get_struct(&self, name: &Name) -> Option<&Struct> {
        for scope in self.iter() {
            if let Some(dt) = scope.structs.get(name) {
                return Some(dt);
            }
        }

        None
    }

    // forward insert type and function definitions
    pub fn forward_insert(&mut self, stmts: &Vec<Stmt>) -> Result<(), SemanticError> {
        for stmt in stmts {
            match stmt.kind {
                StmtKind::FuncDecl(ref name, ref rt, ref params, _) => {
                    let params = params.into_iter().map(|field| &field.0).cloned().collect();
                    self.insert_binding(name, DataType::Func(Box::new(rt.clone()), params))?;
                },
                StmtKind::StructDecl(ref name, ref fields) => {
                    self.insert_struct(name, Struct::new(fields.to_vec()))?;
                },
                _ => {}
            }
        }

        Ok(())
    }
}

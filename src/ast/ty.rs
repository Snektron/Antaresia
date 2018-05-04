use std::marker::PhantomData;
use std::fmt;
use ast::Name;
use check::{CheckType, Unchecked};
use utility;
use parser::Span;
use std::cmp::PartialEq;

#[derive(Debug, Clone)]
pub struct Ty<C = Unchecked>
where C: CheckType {
    pub span: Span,
    pub kind: TyKind<C>,
    _checktype: PhantomData<C>
}

impl<C> Ty<C>
where C: CheckType {
    pub fn new(span: Span, kind: TyKind<C>) -> Ty<C> {
        Ty {
            span,
            kind,
            _checktype: PhantomData
        }
    }

    pub fn void(span: Span) -> Ty<C> {
        Ty::new(span, TyKind::Void)
    }

    pub fn dereference(self) -> Option<Ty<C>> {
        match self.kind {
            TyKind::Ptr(pointee) => Some(*pointee),
            _ => None
        }
    }

    pub fn reference(self) -> Ty<C> {
        Ty::new(self.span.clone(), TyKind::Ptr(Box::new(self)))
    }

    pub fn is_void(&self) -> bool {
        match self.kind {
            TyKind::Void => true,
            _ => false
        }
    }
}

impl<C> PartialEq for Ty<C>
where C: CheckType {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl<C> fmt::Display for Ty<C>
where C: CheckType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            TyKind::U8 => write!(f, "u8"),
            TyKind::Void => write!(f, "void"),
            TyKind::Alias(ref name) => write!(f, "{}", name),
            TyKind::Ptr(ref pointee) => write!(f, "{}*", pointee),
            TyKind::Func(ref func) => func.fmt(f),
            TyKind::Struct(ref strukt) => strukt.fmt(f),
            TyKind::Paren(ref inner) => write!(f, "({})", inner),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TyKind<C = Unchecked>
where C: CheckType {
    U8,
    Void,
    Alias(Name),
    Ptr(Box<Ty<C>>),
    Func(Box<FuncTy<C>>),
    Struct(Box<StructTy<C>>),
    Paren(Box<Ty<C>>) // for pretty-printing
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field<C = Unchecked>
where C: CheckType {
    pub name: Name,
    pub ty: Ty<C>
}

impl<C> Field<C>
where C: CheckType {
    pub fn new(name: Name, ty: Ty<C>) -> Self {
        Field {
            name,
            ty
        }
    }
}

impl<C> fmt::Display for Field<C>
where C: CheckType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.ty)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncTy<C = Unchecked>
where C: CheckType {
    pub params: Vec<Ty<C>>,
    pub return_ty: Ty<C>
}

impl<C> FuncTy<C>
where C: CheckType {
    pub fn new(params: Vec<Ty<C>>, return_ty: Ty<C>) -> FuncTy<C> {
        FuncTy {
            params,
            return_ty
        }
    }
}

impl<C> fmt::Display for FuncTy<C>
where C: CheckType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "func(")?;
        utility::write_comma_seperated(f, self.params.iter())?;
        write!(f, ") -> {}", self.return_ty)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructTy<C = Unchecked>
where C: CheckType {
    pub fields: Vec<Field<C>>
}

impl<C> StructTy<C>
where C: CheckType {
    pub fn new(fields: Vec<Field<C>>) -> StructTy<C> {
        StructTy {
            fields
        }
    }
}

impl<C> fmt::Display for StructTy<C>
where C: CheckType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "struct {{")?;
        utility::write_comma_seperated(f, self.fields.iter())?;
        write!(f, "}}")
    }
}
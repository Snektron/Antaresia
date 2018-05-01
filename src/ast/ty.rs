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
    pub fn new(span: Span, kind: TyKind<C>) -> Self {
        Ty {
            span,
            kind,
            _checktype: PhantomData
        }
    }

    pub fn dereference(self) -> Option<Self> {
        match self.kind {
            TyKind::Ptr(pointee) => Some(*pointee),
            _ => None
        }
    }

    pub fn reference(self) -> Self {
        Ty::new(self.span.clone(), TyKind::Ptr(Box::new(self)))
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
            TyKind::Func(ref ret_type, ref params) => {
                write!(f, "func(")?;
                utility::write_comma_seperated(f, params.iter())?;
                write!(f, ") -> {}", ret_type)
            },
            TyKind::Paren(ref inner) => write!(f, "({})", inner),
        }
    }
}

impl<C> PartialEq for Ty<C>
where C: CheckType {
    fn eq(&self, other: &Self) -> bool {
        self.kind.eq(&other.kind)
    }
}

#[derive(Debug, Clone)]
pub enum TyKind<C = Unchecked>
where C: CheckType {
    U8,
    Void,
    Alias(Name),
    Ptr(Box<Ty<C>>),
    Func(Box<Ty<C>>, Vec<Ty<C>>),
    Paren(Box<Ty<C>>) // for pretty-printing
}

impl<C> PartialEq for TyKind<C>
where C: CheckType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&TyKind::U8, &TyKind::U8) => true,
            (&TyKind::Void, &TyKind::Void) => true,
            (&TyKind::Alias(ref l), &TyKind::Alias(ref r)) => l == r,
            (&TyKind::Ptr(ref l), &TyKind::Ptr(ref r)) => l == r,
            (&TyKind::Func(ref ln, ref lp), &TyKind::Func(ref rn, ref rp)) => ln == rn && lp == rp,
            (&TyKind::Paren(ref l), &TyKind::Paren(ref r)) => l == r,
            _ => false
        }
    } 
}

#[derive(Debug, Clone)]
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

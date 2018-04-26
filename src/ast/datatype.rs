use std::marker::PhantomData;
use std::fmt;
use ast::Name;
use check::{CheckType, Unchecked};
use utility;
use parser::Span;
use std::cmp::PartialEq;

#[derive(Debug, Clone)]
pub struct DataType<C = Unchecked>
where C: CheckType {
    pub span: Span,
    pub kind: DataTypeKind<C>,
    _checktype: PhantomData<C>
}

impl<C> DataType<C>
where C: CheckType {
    pub fn new(span: Span, kind: DataTypeKind<C>) -> Self {
        DataType {
            span,
            kind,
            _checktype: PhantomData
        }
    }
}

impl<C> fmt::Display for DataType<C>
where C: CheckType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            DataTypeKind::U8 => write!(f, "u8"),
            DataTypeKind::Void => write!(f, "void"),
            DataTypeKind::Alias(ref name) => write!(f, "{}", name),
            DataTypeKind::Ptr(ref pointee) => write!(f, "{}*", pointee),
            DataTypeKind::Func(ref ret_type, ref params) => write!(f, "func({}) -> {}", params, ret_type),
            DataTypeKind::Paren(ref inner) => write!(f, "({})", inner),
        }
    }
}

impl<C> PartialEq for DataType<C>
where C: CheckType {
    fn eq(&self, other: &Self) -> bool {
        self.kind.eq(&other.kind)
    } 
}

#[derive(Debug, Clone)]
pub enum DataTypeKind<C = Unchecked>
where C: CheckType {
    U8,
    Void,
    Alias(Name),
    Ptr(Box<DataType<C>>),
    Func(Box<DataType<C>>, DataTypeVec<C>),
    Paren(Box<DataType<C>>) // for pretty-printing
}

impl<C> PartialEq for DataTypeKind<C>
where C: CheckType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&DataTypeKind::U8, &DataTypeKind::U8) => true,
            (&DataTypeKind::Void, &DataTypeKind::Void) => true,
            (&DataTypeKind::Alias(ref l), &DataTypeKind::Alias(ref r)) => l == r,
            (&DataTypeKind::Ptr(ref l), &DataTypeKind::Ptr(ref r)) => l == r,
            (&DataTypeKind::Func(ref ln, ref lp), &DataTypeKind::Func(ref rn, ref rp)) => ln == rn && lp == rp,
            (&DataTypeKind::Paren(ref l), &DataTypeKind::Paren(ref r)) => l == r,
            _ => false
        }
    } 
}

#[derive(Debug, Clone)]
pub struct Field<C = Unchecked>
where C: CheckType {
    pub name: Name,
    pub datatype: DataType<C>
}

impl<C> Field<C>
where C: CheckType {
    pub fn new(name: Name, datatype: DataType<C>) -> Self {
        Field {
            name,
            datatype
        }
    }
}

impl<C> fmt::Display for Field<C>
where C: CheckType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.datatype)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DataTypeVec<C = Unchecked>
where C: CheckType {
    inner: Vec<DataType<C>>
}

impl<C> DataTypeVec<C>
where C: CheckType {
    pub fn new(types: Vec<DataType<C>>) -> Self {
        DataTypeVec {
            inner: types
        }
    }
}

impl<C> fmt::Display for DataTypeVec<C>
where C: CheckType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        utility::write_comma_seperated(f, self.inner.iter())
    }
}

// #[derive(Debug, Clone, PartialEq)]
// pub struct FieldVec<C = Unchecked>
// where C: CheckType {
//     inner: Vec<Field<C>>
// }

// impl<C> FieldVec<C>
// where C: CheckType {
//     pub fn new(fields: Vec<Field<C>>) -> Self {
//         FieldVec {
//             inner: fields
//         }
//     }
// }

// impl<C> fmt::Display for FieldVec<C>
// where C: CheckType {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         utility::write_comma_seperated(f, self.inner.iter())
//     }
// }

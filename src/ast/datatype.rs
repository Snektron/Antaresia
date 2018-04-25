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
            DataTypeKind::Func(ref ret_type, ref params) => {
                write!(f, "func(")?;
                utility::write_comma_seperated(f, params.iter())?;
                write!(f, ") -> {}", ret_type)
            },
            DataTypeKind::Paren(ref inner) => write!(f, "({})", inner),
        }
    }
}

impl<C> PartialEq for DataType<C>
where C: CheckType {
    fn eq(&self, other: &Self) -> bool {
        self.kind.eq(&other)
    } 
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataTypeKind<C = Unchecked>
where C: CheckType {
    U8,
    Void,
    Alias(Name),
    Ptr(Box<DataType<C>>),
    Func(Box<DataType<C>>, Vec<DataType<C>>),
    Paren(Box<DataType<C>>) // for pretty-printing
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

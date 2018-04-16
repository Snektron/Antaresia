use ast::Name;

#[derive(Clone, Debug, PartialEq)]
pub enum DataType {
    U8,
    Void,
    Alias(Name),
    Ptr(Box<DataType>),
    Func(Box<DataType>, Vec<DataType>),
}

#[derive(Debug)]
pub struct Field(pub DataType, pub Name);

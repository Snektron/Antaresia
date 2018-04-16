use ast::Name;

#[derive(Clone, Debug, PartialEq)]
pub enum DataType {
    U8,
    Void,
    Alias(Name),
    Ptr(Box<DataType>),
    Func(Box<DataType>, Vec<DataType>),
}

#[derive(Clone, Debug)]
pub struct Field(pub DataType, pub Name);

pub struct Struct {
    fields: Vec<Field>
}

impl Struct {
    pub fn new(fields: Vec<Field>) -> Self {
        Struct {
            fields
        }
    }
}
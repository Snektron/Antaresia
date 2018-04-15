#[derive(Clone, Debug, PartialEq)]
pub enum DataType {
    U8,
    Void,
    Alias(String),
    Ptr(Box<DataType>)
}

#[derive(Debug)]
pub struct Field(pub DataType, pub String);

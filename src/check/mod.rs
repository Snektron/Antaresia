use datatype::DataType;

pub trait Check {}

pub struct UnChecked {}

impl Check for UnChecked {}

pub struct Checked {
    datatype: DataType
}

impl Check for Checked {}

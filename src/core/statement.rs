use std::mem::size_of;

use crate::core::record::Record;

#[derive(Debug)]
pub enum Type {
    INSERT,
    SELECT,
}

#[derive(Debug)]
pub struct Statement {
    pub stype: Type,
    pub record: Option<Record>,
}

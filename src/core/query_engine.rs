use std::ptr;

use crate::core::{
    record::{self, Record},
    statement::{self, Statement},
    table::Table,
};

#[derive(Debug)]
pub enum Error {
    TABLE_FULL,
    SLOT_FAILURE,
    GENERIC,
}

fn execute_insert(table: &mut Box<Table>, statement: &Statement) -> Result<(), Error> {
    println!("Executing insert");

    if table.is_full() {
        eprintln!("Table is full");
        return Err(Error::TABLE_FULL);
    }

    let record_num = table.num_records + 1;

    let record = statement.record.as_ref().unwrap();

    let mut slot_ptr = match table.record_slot(record_num) {
        Some(slot_ptr) => slot_ptr,
        None => {
            eprintln!("Failed to get record slot");
            return Err(Error::SLOT_FAILURE);
        }
    };

    println!("Slot address: {:p}", slot_ptr);

    // Write record to memeory slot
    unsafe {
        ptr::copy_nonoverlapping(record as *const Record, slot_ptr as *mut Record, 1);
    }

    table.num_records = record_num;

    println!("Record inserted in table");

    return Ok(());
}

fn execute_select(table: &mut Box<Table>, statement: &Statement) {
    println!("Executing select");
}

pub fn handler(table: &mut Box<Table>, statement: &Statement) {
    match statement.stype {
        statement::Type::INSERT => {
            execute_insert(table, statement);
        }
        statement::Type::SELECT => {
            execute_select(table, statement);
        }
    }
}

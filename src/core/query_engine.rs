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

    let record_num = table.num_records;

    let record = statement.record.as_ref().unwrap();

    let mut slot_ptr = match table.record_slot(record_num) {
        Some(slot_ptr) => slot_ptr,
        None => {
            eprintln!("Failed to get record slot");
            return Err(Error::SLOT_FAILURE);
        }
    };

    // println!("Slot address: {:p}", slot_ptr);

    // Write record to memeory slot
    unsafe {
        ptr::copy_nonoverlapping(record as *const Record, slot_ptr as *mut Record, 1);
    }

    table.num_records = record_num + 1;

    println!("Record inserted in table");

    return Ok(());
}

fn execute_select(table: &mut Box<Table>, statement: &Statement) -> Result<(), Error> {
    println!("Executing select");

    let mut record: Record;

    for num in 0..table.num_records {
        let mut slot_ptr = match table.record_slot(num) {
            Some(slot_ptr) => slot_ptr,
            None => {
                eprintln!("Failed to get record slot");
                return Err(Error::SLOT_FAILURE);
            }
        };

        // Write record in page to record
        unsafe {
            record = ptr::read(slot_ptr as *const Record);
        }

        println!("  ({})", record)
    }

    Ok(())
}

pub fn handler(table: &mut Box<Table>, statement: &Statement) -> Result<(), Error> {
    match statement.stype {
        statement::Type::INSERT => {
            return execute_insert(table, statement);
        }
        statement::Type::SELECT => {
            return execute_select(table, statement);
        }
    }
}

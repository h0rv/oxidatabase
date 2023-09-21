use std::{
    alloc::{alloc, Layout},
    mem::size_of,
};

use crate::core::{
    page::{self, Page, Pointer},
    record::{self},
};

const MAX_RECORDS: usize = page::MAX_RECORDS;
const MAX_PAGES: usize = 100;
const WORD_SIZE: usize = size_of::<usize>();

#[derive(Debug)]
pub struct Table {
    pub num_records: usize,
    pub pages: [Page; MAX_PAGES],
}

impl Default for Table {
    fn default() -> Self {
        Self {
            num_records: 0,
            pages: [Page::new(); MAX_PAGES],
        }
    }
}

impl Table {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn is_full(self: &Self) -> bool {
        return self.num_records >= MAX_RECORDS;
    }

    pub fn record_slot(self: &mut Self, record_num: usize) -> Pointer {
        let page_num = record_num / page::MAX_RECORDS;

        let mut page = self.pages[page_num];

        if page.pointer.is_none() {
            // Allocate memory for page
            println!("Allocating page number {}", page_num);
            page.alloc();
        }

        // Get page pointer
        let mut page_ptr = match page.pointer {
            Some(page_ptr) => page_ptr,
            None => {
                eprintln!("Error getting page pointer");
                return None;
            }
        };

        // Calculate location of record
        let record_offset = record_num % page::MAX_RECORDS;
        let byte_offset = record_offset * record::SIZE;
        let word_offset = (byte_offset / WORD_SIZE) as isize;

        println!("Page address: {:p}", page_ptr);

        page_ptr = unsafe { page_ptr.offset(word_offset) };

        println!("page addr + word offset ({}) = {:p}", word_offset, page_ptr);

        return Some(page_ptr);
    }
}

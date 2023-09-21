use std::{
    alloc::{alloc, Layout},
    iter,
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
    pub pages: Vec<Page>,
}

impl Default for Table {
    fn default() -> Self {
        let mut pages: Vec<Page> = Vec::with_capacity(MAX_PAGES);
        for _ in 0..MAX_PAGES {
            pages.push(Page::new());
        }

        Self {
            num_records: 0,
            pages: iter::repeat_with(|| Page::new()).take(MAX_PAGES).collect(),
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

        let mut page = unsafe { self.pages.get_unchecked_mut(page_num) }; // Vector is fixed size

        if !page.is_allocated() {
            // Allocate memory for page
            println!("Allocating page number {}", page_num);
            page.allocate();
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
        let byte_offset = (record_offset * record::SIZE) as isize;

        // println!("Page address: {:p}", page_ptr);

        page_ptr = unsafe { page_ptr.offset(byte_offset) };

        // println!("page addr + byte offset ({}) = {:p}", byte_offset, page_ptr);

        return Some(page_ptr);
    }
}

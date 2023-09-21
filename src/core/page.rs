use std::{
    alloc::{alloc, dealloc, Layout},
    mem::{align_of, size_of},
};

use crate::core::record;

pub const SIZE: usize = 4096; // 4 KB
pub const MAX_RECORDS: usize = SIZE / record::SIZE;

pub type Pointer = Option<*mut isize>;

#[derive(Debug, Default)]
pub struct Page {
    pub pointer: Pointer,
}

const PAGE_LAYOUT: Layout = unsafe { Layout::from_size_align_unchecked(SIZE, align_of::<usize>()) };

impl Page {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn is_allocated(&self) -> bool {
        return self.pointer.is_some();
    }

    pub fn allocate(&mut self) {
        let page_ptr = unsafe { alloc(PAGE_LAYOUT) } as *mut isize;
        self.pointer = Some(page_ptr);
    }
}

// This trivial implementation of `drop` adds a print to console.
impl Drop for Page {
    fn drop(&mut self) {
        if self.is_allocated() {
            unsafe { dealloc(self.pointer.unwrap() as *mut u8, PAGE_LAYOUT) };
        }
    }
}

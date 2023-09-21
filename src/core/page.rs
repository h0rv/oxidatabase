use std::{
    alloc::{alloc, Layout},
    mem::{align_of, size_of},
};

use crate::core::record;

pub const SIZE: usize = 4096; // 4 KB
pub const MAX_RECORDS: usize = SIZE / record::SIZE;

pub type Pointer = Option<*mut isize>;

#[derive(Copy, Clone, Debug, Default)]
pub struct Page {
    pub pointer: Pointer,
}

const PAGE_LAYOUT: Layout = unsafe { Layout::from_size_align_unchecked(SIZE, align_of::<usize>()) };

impl Page {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn alloc(self: &mut Self) {
        let page_ptr = unsafe { alloc(PAGE_LAYOUT) } as *mut isize;
        self.pointer = Some(page_ptr);
    }
}

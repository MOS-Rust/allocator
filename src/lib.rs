#![no_std]
extern crate alloc;
extern crate spin;

use core::ops::Deref;
use alloc::alloc::{GlobalAlloc, Layout};
use spin::Mutex;

mod buddy;
mod list;
mod tests;

use buddy::Heap;

pub struct Allocator<const ORDER: usize>(Mutex<Heap<ORDER>>);

impl<const ORDER: usize> Allocator<ORDER> {
    pub const fn new() -> Self {
        Allocator(Mutex::new(Heap::new()))
    }
}

impl<const ORDER: usize> Deref for Allocator<ORDER> {
    type Target = Mutex<Heap<ORDER>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

unsafe impl<const ORDER: usize> GlobalAlloc for Allocator<ORDER> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0
            .lock()
            .alloc(layout)
            .ok()
            .map_or(core::ptr::null_mut(), |ptr| ptr.as_ptr())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0
            .lock()
            .dealloc(core::ptr::NonNull::new_unchecked(ptr), layout)
    }
}

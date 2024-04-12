#![no_std]
#![cfg_attr(feature = "const_fn", feature(const_mut_refs, const_fn_fn_ptr_basics))]

extern crate alloc;

use alloc::alloc::{GlobalAlloc, Layout};
mod list;
mod buddy;
mod tests;
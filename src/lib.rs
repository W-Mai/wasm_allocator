#![no_std]
#![no_main]
#![feature(alloc_error_handler, core_intrinsics, const_cmp)]
#![feature(const_mut_refs)]

mod utils;

// use core::arch::wasm32;
use core::alloc::{GlobalAlloc, Layout};
use core::cmp::{max, min};

const HEAP_SIZE: usize = 8 * 1024 * 1024; // 1 MB

static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

static mut HEAP_TOP: usize = 0;

pub struct Heap;

unsafe impl GlobalAlloc for Heap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        if HEAP_TOP < HEAP_SIZE {
            let ptr = HEAP.as_mut_ptr().add(HEAP_TOP);
            HEAP_TOP += size;
            return ptr;
        }
        core::ptr::null_mut()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        if ptr.is_null() {
            return;
        }
    }
}

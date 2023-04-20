#![no_std]
#![no_main]
#![feature(alloc_error_handler, core_intrinsics)]

mod utils;

// use core::arch::wasm32;
use core::alloc::{GlobalAlloc, Layout};

const HEAP_SIZE: usize = include!(concat!(env!("OUT_DIR"), "/WASM_ALLOCATOR_HEAP_SIZE"));

static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

static mut HEAP_TOP: usize = 0;

pub struct Heap;

#[allow(dead_code)]
unsafe impl GlobalAlloc for Heap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        if HEAP_TOP < HEAP_SIZE {
            let ptr = HEAP.as_mut_ptr().add(HEAP_TOP);
            let aligned_size = ptr.align_offset(align) + size; // get aligned size offset
            if HEAP_TOP + aligned_size <= HEAP_SIZE {
                HEAP_TOP += aligned_size;
                return ptr;
            }
        }

        core::ptr::null_mut()
    }

    // current doesn't support dealloc
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        if ptr.is_null() {}
    }
}

impl Heap {
    /// # Safety
    ///
    /// This function is unsafe because it is not thread-safe.
    pub fn get_heap_top() -> usize {
        unsafe { HEAP_TOP }
    }
}

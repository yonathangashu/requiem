extern crate alloc;

use core::alloc::GlobalAlloc;
use core::cell::UnsafeCell;
use core::ptr::null_mut;

pub struct BumpAllocator {
    bump_ptr: UnsafeCell<usize>,
    end_addr: UnsafeCell<usize>,
}

// UEFI is single-threaded by default so we can confidently say it wont be accessed from multiple
// threads
unsafe impl Sync for BumpAllocator {}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        let aligned_addr = unsafe { (*self.bump_ptr.get() + align - 1) & !(align - 1) };
        let aligned_ptr: *mut u8 = aligned_addr as *mut u8;

        // Check if we have space in our allocated pages and advance bump ptr, if not return null
        unsafe {
            if aligned_addr + size <= *self.end_addr.get() {
                *self.bump_ptr.get() = aligned_addr + size;
            } else {
                return null_mut();
            }
        }
        aligned_ptr
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {}
}

impl Default for BumpAllocator {
    fn default() -> Self {
        Self::new()
    }
}

impl BumpAllocator {
    pub const fn new() -> Self {
        Self {
            bump_ptr: UnsafeCell::new(0),
            end_addr: UnsafeCell::new(0),
        }
    }
    pub fn init(&self, bump_ptr_addr: usize, size: usize) {
        let end_addr = bump_ptr_addr + size;
        unsafe {
            *self.bump_ptr.get() = bump_ptr_addr;
            *self.end_addr.get() = end_addr;
        }
    }
    pub fn get_bump_ptr(&self) -> usize {
        unsafe { *self.bump_ptr.get() }
    }

    pub fn get_end_addr(&self) -> usize {
        unsafe { *self.end_addr.get() }
    }
}

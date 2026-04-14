extern crate alloc;

use alloc::boxed::Box;
use core::cell::UnsafeCell;

pub static PLATFORM_OPS: PlatformOp = PlatformOp::new();

pub struct PlatformOp {
    platform_op: UnsafeCell<Option<Box<dyn PlatformOps>>>,
}

unsafe impl Sync for PlatformOp {}

impl PlatformOp {
    const fn new() -> Self {
        Self {
            platform_op: UnsafeCell::new(None),
        }
    }
    pub fn init(&self, platform_ops: Box<dyn PlatformOps>) {
        unsafe {
            *self.platform_op.get() = Some(platform_ops);
        }
    }
    pub fn get(&self) -> &dyn PlatformOps {
        unsafe {
            match self.platform_op.get().as_ref().unwrap().as_deref() {
                Some(platformop) => platformop,
                None => panic!("Platform ops not found, likely uninitialized"),
            }
        }
    }
}
pub trait PlatformOps {
    fn virt_to_physical(&self, virt_addr: usize) -> usize;
    fn run_on_all_processors(&self, function: fn());
}

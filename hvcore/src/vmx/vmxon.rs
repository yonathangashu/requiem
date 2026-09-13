extern crate alloc;

use alloc::boxed::Box;

use crate::instructions::rdmsr;

struct VMXONRegion {
    region: Box<[u8; 4096]>,
}

impl VMXONRegion {
    fn new() -> Self {
        let zeroed_region = Box::<[u8; 4096]>::new_zeroed();
        let mut zeroed_region = unsafe { zeroed_region.assume_init() };

        let mut ia32_vmx_basic: u32 = rdmsr(0x480) as u32;
        ia32_vmx_basic &= !(1 << 31);
        let ia32_vmx_basic_bytes = ia32_vmx_basic.to_le_bytes();

        zeroed_region[0..4].copy_from_slice(&ia32_vmx_basic_bytes);

        Self {
            region: zeroed_region,
        }
    }
}

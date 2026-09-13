extern crate alloc;

use crate::instructions::rdmsr;
use crate::vmx::PLATFORM_OPS;

use alloc::boxed::Box;

#[repr(align(4096))]
struct Align4096([u8; 4096]);

pub struct VMXONRegion {
    region: Box<Align4096>,
}

impl VMXONRegion {
    pub fn new() -> Self {
        let zeroed_region = Box::<Align4096>::new_zeroed();
        let mut zeroed_region = unsafe { zeroed_region.assume_init() };

        let mut ia32_vmx_basic: u32 = rdmsr(0x480) as u32;
        ia32_vmx_basic &= !(1 << 31);
        let ia32_vmx_basic_bytes = ia32_vmx_basic.to_le_bytes();

        zeroed_region.0[0..4].copy_from_slice(&ia32_vmx_basic_bytes);

        Self {
            region: zeroed_region,
        }
    }
    pub fn get_phys_addr(&self) -> usize {
        let virt_region = self.region.0.as_ptr() as usize;
        PLATFORM_OPS.get().virt_to_physical(virt_region)
    }
}

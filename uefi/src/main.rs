#![no_std]
#![no_main]

extern crate alloc;

mod uefi_ops;

use alloc::boxed::Box;
use hvcore::platform_ops::PLATFORM_OPS;
use hvcore::vmx::virtualize_system;
use hvcore::vmx::vmxon::VMXONRegion;
use hvcore::*;
use log::info;
use uefi::boot::*;
use uefi::prelude::*;
use uefi::print;

use crate::uefi_ops::UefiOps;

#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator::new();

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    print!("Requiem loading...\n");

    let pages_result = allocate_pages(
        AllocateType::AnyPages,
        MemoryType::RUNTIME_SERVICES_DATA,
        1024,
    );

    let pages = match pages_result {
        Ok(page) => page,
        Err(error) => {
            print!("{}", error.status());
            return error.status();
        }
    };

    info!("Pointer to start of allocated chunk: {:#x}", pages.addr());
    ALLOCATOR.init(pages.addr().into(), ALLOCATION_SIZE);
    info!("Bump pointer: {:#x}", ALLOCATOR.get_bump_ptr());
    info!("End addresses: {:#x}", ALLOCATOR.get_end_addr());

    let uefi_platform = UefiOps::new();
    PLATFORM_OPS.init(Box::new(uefi_platform));

    let uefi_end_addr = PLATFORM_OPS
        .get()
        .virt_to_physical(ALLOCATOR.get_end_addr());
    info!("UEFI Translation of End Addr: {:#x}", uefi_end_addr);

    //TODO: Remove this VMXON debugging chunk; wire it into initialize_core
    let vmxon_region = vmx::vmxon::VMXONRegion::new();
    let vmxon_phys_addr = vmxon_region.get_phys_addr();
    info!("VMXON Physical Address: {:#x}", vmxon_phys_addr);
    info!("New bump pointer: {:#x}", ALLOCATOR.get_bump_ptr());

    // Verify system support for virtualization
    // Initialize all cores w/ vmx_enable and set CR4.VMXXE bit
    virtualize_system().unwrap();
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }

    Status::SUCCESS
}

#![no_std]
#![no_main]

extern crate alloc;

mod uefi_ops;

use alloc::boxed::Box;
use hvcore::platform_ops::PLATFORM_OPS;
use hvcore::vmx::virtualize_system;
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
    print!("Requiem loading...");

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

    info!("Pointer to start of allocated chunk: {:#?}", pages.addr());
    ALLOCATOR.init(pages.addr().into(), ALLOCATION_SIZE);
    info!("Bump pointer: {:#?}", ALLOCATOR.get_bump_ptr());
    info!("End addresses: {:#?}", ALLOCATOR.get_end_addr());

    let uefi_platform = UefiOps::new();
    PLATFORM_OPS.init(Box::new(uefi_platform));

    let uefi_end_addr = PLATFORM_OPS
        .get()
        .virt_to_physical(ALLOCATOR.get_end_addr());
    info!("UEFI Translation of End Addr: {:#?}", uefi_end_addr);

    // Verify system support for virtualization
    // Initialize all cores w/ vmx_enable and set CR4.VMXXE bit
    virtualize_system().unwrap();

    Status::SUCCESS
}

#![no_std]
#![no_main]

mod uefi_ops;

use hvcore::ALLOCATION_SIZE;
use hvcore::allocator::ALLOCATOR;
use log::info;
use uefi::boot::*;
use uefi::prelude::*;
use uefi::print;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    print!("equiem loading...");

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

    Status::SUCCESS
}

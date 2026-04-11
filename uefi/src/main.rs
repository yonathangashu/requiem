#![no_std]
#![no_main]

use hvcore::ALLOCATION_SIZE;
use hvcore::allocator::ALLOCATOR;
use uefi::boot::*;
use uefi::prelude::*;
use uefi::print;

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

    print!("Pointer to start of allocated chunk: {:#?}", pages);
    ALLOCATOR.init(pages.addr().into(), ALLOCATION_SIZE);
    print!("Bump pointer: {:#?}", ALLOCATOR.get_bump_ptr());
    print!("End addresses: {:#?}", ALLOCATOR.get_end_addr());

    Status::SUCCESS
}

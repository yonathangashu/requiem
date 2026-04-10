#![no_std]
#![no_main]

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

    Status::SUCCESS
}

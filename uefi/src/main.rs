#![no_std]
#![no_main]

use uefi::prelude::*;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    uefi::println!("Requiem loading...");
    Status::SUCCESS
}

#![no_std]

pub mod allocator;
pub mod instructions;
pub mod vmx;

pub const PAGE_SIZE: usize = 4096;
pub const ALLOCATION_SIZE: usize = 1024 * 4096;

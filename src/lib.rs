#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod interrupts;
pub mod qemu;
pub mod serial;
pub(crate) mod test;
pub mod vga_buffer;
pub mod volatile;

pub use qemu::*;
pub use test::*;

/// Handles the initialization process.
pub fn init() {
    interrupts::init_idt();
}

/// Test entrypoint
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();

    loop {}
}

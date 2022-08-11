#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(based_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use based_os::{println, test_panic_handler};

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info);
}

#[test_case]
fn println_should_not_panic() {
    println!("println did not panic!");
}

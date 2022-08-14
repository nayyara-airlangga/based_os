#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(based_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use based_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to BasedOS");
    println!("v0.1.0");

    // Run initializers
    based_os::init();

    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    based_os::test_panic_handler(info);
}

#[cfg(test)]
mod tests {
    #[test_case]
    fn trivial_assertion() {
        assert_eq!(1, 1);
    }
}

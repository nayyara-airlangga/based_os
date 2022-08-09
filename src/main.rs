#![no_std]
#![no_main]

mod vga_buffer;
mod volatile;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to BasedOS v0.1.0");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{_info}");

    loop {}
}

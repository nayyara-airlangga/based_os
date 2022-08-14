//! BasedOS testing utilities

use core::{any::type_name, panic::PanicInfo};

use crate::{qemu_exit, serial_print, serial_println, QemuExitCode};

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) -> () {
        serial_print!("{} ... ", type_name::<T>());
        self();
        serial_println!("ok");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests...", tests.len());

    for test in tests {
        test.run();
    }

    qemu_exit(QemuExitCode::Success)
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("failed\n");
    serial_println!("Error: {}\n", info);

    qemu_exit(QemuExitCode::Failed);

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info);
}

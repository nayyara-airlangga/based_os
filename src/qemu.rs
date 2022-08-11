use x86_64::instructions::port::Port;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

/// x86 (generally) free to use port
const PORT_ADDRESS: u16 = 0xf4;

pub fn qemu_exit(code: QemuExitCode) {
    unsafe {
        let mut port = Port::new(PORT_ADDRESS);
        port.write(code as u32)
    }
}

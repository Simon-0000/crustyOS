#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
pub mod serial;
pub mod vga_buffer; 


const DEVICE_ADDRESS: u16 = 0xf4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub trait Testable {
    fn run(&self) -> ();
}
impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[OK]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(ExitCode::Success);
}
pub fn exit_qemu(code: ExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(DEVICE_ADDRESS);
        port.write(code as u32);
    }
}

pub fn test_panic(info: &PanicInfo) -> ! {
    serial_println!("[FAILED]\nERROR: {}", info);
    exit_qemu(ExitCode::Failed);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic(info);
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop{};
 }


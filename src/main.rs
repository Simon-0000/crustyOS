#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use core::fmt::Write;
use vga_buffer::{Color, ColorCode, WRITER};
use x86_64::instructions::port::Port;
use test_tools::Testable;
mod vga_buffer;
mod serial;
mod test_tools;

const DEVICE_ADDRESS: u16= 0xf4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
enum ExitCode{
    Success = 0x10,
    Failed = 0x11
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {

    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    Exit(ExitCode::Success);
}
#[test_case]
fn test_trivial_1(){
    assert_eq!(1,1);
}

#[test_case]
fn test_trivial_2(){
    assert_eq!(1,0);
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cprintln!(Color::Red,"\n{}",info);
    loop {}
}
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\nERROR: {}",info);
    Exit(ExitCode::Failed);
    loop {}
}

fn Exit(code : ExitCode)
{
    unsafe{
        let mut port = Port::new(DEVICE_ADDRESS);
        port.write(code as u32);
    }
}

#[no_mangle]
pub extern "C" fn _start()-> ! {
    // let port =

    #[cfg(test)]
    {
        test_main();
    }
    #[cfg(not(test))]
    {
        println!("running...");
        Exit(ExitCode::Success);
    }

    loop{}
}


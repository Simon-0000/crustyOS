#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use core::fmt::Write;
use vga_buffer::{Color, ColorCode, WRITER};
use x86_64::instructions::port::Port;
mod vga_buffer;

const DEVICE_ADDRESS: u16= 0xf4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
enum ExitCode{
    Success = 0x10,
    Failed = 0x11
}
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    Exit(ExitCode::Success);
}

#[test_case]
fn testing(){
    cprintln!(Color::Green,"testing function called(green text)");
    println!("default text color");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cprintln!(Color::Red,"\n{}",info);
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


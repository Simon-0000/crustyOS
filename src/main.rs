#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use core::fmt::Write;
use vga_buffer::{Color, ColorCode, WRITER};


#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn testing(){
    cprintln!(Color::Green,"testing function called(green text)");
    println!("default text color");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cprintln!(Color::Red,"\n{}",info);
    loop {}
}

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start()-> ! {
    #[cfg(test)]
    {
        test_main();
        loop{}
    }
    println!("okkok{}",3+5);
    print!("Hello");
    print!(" world");
    panic!("Some panic message");
    loop{}
}


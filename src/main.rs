#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crustyOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use crustyOS::{*,self,println,cprintln,print};



#[test_case]
fn test_trivial_1() {
    assert_eq!(1, 1);
}

#[test_case]
fn test_trivial_2() {
    assert_eq!(1, 0);
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cprintln!(Color::Red, "\n{}", info);
    loop {}
}
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic(info);
}



#[no_mangle]
pub extern "C" fn _start() -> ! {
    // let port =

    #[cfg(test)]
    {
        test_main();
    }
    #[cfg(not(test))]
    {
        println!("running...");
        exit_qemu(ExitCode::Success);
    }
    loop {}
}

#![no_std]
#![no_main]
use core::panic::PanicInfo;
use core::fmt::Write;

use vga_buffer::{ColorCode, Writer, WRITER};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let original_color =  WRITER.lock().color_code;
    let error_color = ColorCode::from_u8(vga_buffer::Color::Red as u8,  WRITER.lock().color_code.get_background_value());
    WRITER.lock().color_code = error_color;
    println!("{}", info);
    WRITER.lock().color_code = original_color;//might not be necessary to put the original color back but idk yet
    loop {}
}

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start()-> ! {
    println!("okkok{}",3+5);
    print!("Hello");
    print!(" world");
    println!();
    panic!("Some panic message");
    loop{}
}


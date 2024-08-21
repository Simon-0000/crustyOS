#![no_std]
#![no_main]
use core::panic::PanicInfo;
use core::fmt::Write;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start()-> ! {
    // let message = b"Hello world!";
    // let buffer_address = 0xb8000 as *mut u8;
    // for(i,&c) in message.iter().enumerate(){
    //     unsafe{
    //         *buffer_address.offset(i as isize * 2) = c;
    //         *buffer_address.offset(i as isize * 2 + 1) = 0xb;
            
    //     }
    // }
    // write!(*vga_buffer::WRITER.lock(), "The numbers are \n{} and {}", 42, 1.0/3.0).unwrap();
    println!("okkok{}",3+5);
    print!("Hello");
    print!(" world");
    println!();
    loop{}
}


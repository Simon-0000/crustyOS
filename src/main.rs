#![no_std]
#![no_main]
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start()-> ! {
    let message = b"Hello world!";
    let buffer_address = 0xb8000 as *mut u8;
    for(i,&c) in message.iter().enumerate(){
        unsafe{
            *buffer_address.offset(i as isize * 2) = c;
            *buffer_address.offset(i as isize * 2 + 1) = 0xb;
            
        }
    }
    loop{}
}



#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod hello;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    hello::hello();
    loop {
        
    }
}

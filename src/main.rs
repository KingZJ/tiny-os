#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod hello;
mod vga_buffer;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {
        
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // hello::hello();
    // vga_buffer::print_something();
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();
    println!("\nHello world{}", "!");
    
    panic!("Some panic message");
    loop {
        
    }
}

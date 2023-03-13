#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(tiny_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use tiny_os::*;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // hello::hello();
    // vga_buffer::print_something();
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(
        vga_buffer::WRITER.lock(),
        ", some numbers: {} {}",
        42,
        1.337
    )
    .unwrap();
    println!("\nHello world{}", "!");

    // panic!("Some panic message");

    #[cfg(test)]
    test_main();
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    tiny_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    // print!("trivial assertion... ");
    // serial_print!("trivial assertion... ");
    assert_eq!(1, 1);
    // println!("[Ok]");
    // serial_println!("[Ok]");
    // loop {

    // }
}

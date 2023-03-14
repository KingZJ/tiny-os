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

    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    // write!(
    //     vga_buffer::WRITER.lock(),
    //     ", some numbers: {} {}",
    //     42,
    //     1.337
    // )
    // .unwrap();

    println!("\nHello world{}", "!");

    init(); //初始化 idt

    // x86_64::instructions::interrupts::int3();  // invoke a breakpoint exception

    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;  // page fault
    // }

    // fn stack_overflow() {
    //     stack_overflow();
    // }
    // stack_overflow();

    // panic!("Some panic message");

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    // loop {
    //     for _ in 0..10000 {}
    //     print!("-");
    // }
    hlt_loop(); // CPU 可暂停的无限循环
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    // loop {}
    hlt_loop();
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

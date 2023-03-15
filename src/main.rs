#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(tiny_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};
use tiny_os::*;

entry_point!(kernel_main);
// #[no_mangle]
// pub extern "C"
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // hello::hello();
    // vga_buffer::print_something();

    // vga_run();

    println!("\nHello world{}", "!");

    init(); //初始化 idt

    // x86_64::instructions::interrupts::int3();  // invoke a breakpoint exception

    // panic_page_fault();
    // print_level4_page_table();
    // print_l4_page_table(boot_info);

    print_virt2phys(boot_info);

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

#[allow(dead_code)]
fn vga_run() {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(
        vga_buffer::WRITER.lock(),
        ", some numbers: {} {}",
        42,
        1.337
    )
    .unwrap();
}

#[allow(dead_code)]
fn panic_page_fault() {
    let ptr = 0x204f49 as *mut u32; // 代码段地址
    unsafe {
        // *(0xdeadbeef as *mut u64) = 42;  // page fault
        let x = *ptr; // 允许读
        println!("{:?}", x);
    }
    println!("read worked");

    unsafe {
        *ptr = 42; // 不允许写
    }
    println!("write worked");
}

#[allow(dead_code)]
fn print_level4_page_table() {
    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!(
        "level 4 page table at {:?}",
        level_4_page_table.start_address()
    );
}

#[allow(dead_code)]
fn print_l4_page_table(boot_info: &'static BootInfo) {
    use x86_64::VirtAddr;

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { memory::active_level_4_table(phys_mem_offset) };
    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("l4 entry {}: {:?}", i, entry);
        }
    }
}

fn print_virt2phys(boot_info: &'static BootInfo) {
    use x86_64::VirtAddr;
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let addresses = [
        0xb8000, // vga buffer page
        0x201008,
        0x0100_0020_1a10,
        boot_info.physical_memory_offset,
    ];

    for &addr in &addresses {
        let virt = VirtAddr::new(addr);
        let phys = unsafe { memory::translate_addr(virt, phys_mem_offset) };
        println!("{:?} -> {:?}", virt, phys);
    }
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

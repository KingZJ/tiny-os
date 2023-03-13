#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

/// 模拟std library should_panic macro
use core::panic::PanicInfo;
use tiny_os::*;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]\n");
    exit_qemu(QemuExitCode::Success);

    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    for test in tests {
        test();
        serial_println!("[test did not panic]\n");
        exit_qemu(QemuExitCode::Failed);
    }

    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn should_fail() {
    serial_print!("should_panic::should_fail... ");
    assert_eq!(0, 1);
}

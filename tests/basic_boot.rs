#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(rosix::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rosix::println;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rosix::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[test_case]
fn test_println() {
    println!("test_println output");
}

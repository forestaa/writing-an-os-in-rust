#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(writing_an_os_in_rust::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use writing_an_os_in_rust::println;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    writing_an_os_in_rust::debug::print_debug_info();
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default

    println!("Hello World{}", "!");

    writing_an_os_in_rust::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    writing_an_os_in_rust::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    writing_an_os_in_rust::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    writing_an_os_in_rust::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

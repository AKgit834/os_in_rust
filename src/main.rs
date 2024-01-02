#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use os::println;

fn stack_overflow(){
    stack_overflow();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {

    os::init();

    println!("Hello Aaditya{}", "!");

    stack_overflow();

    /*unsafe{
        *(0xdeadbeef as *mut u8) = 42;
    };*/

    #[cfg(test)]
    test_main();

    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info)
}

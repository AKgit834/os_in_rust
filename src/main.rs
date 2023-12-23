#![no_std] //dont link to any standard Rust library.
#![no_main] //Disables all rust level entry points.

use core::panic::PanicInfo;

#[panic_handler] 
// this function is called on panic.
fn panic(_panic: &PanicInfo) -> ! {
loop {}
}

#[no_mangle] // dont mangle the name of this function.
//Entry point as the linker looks for function named _start by default. 
pub extern "C" fn _start() -> ! {
loop {}
}

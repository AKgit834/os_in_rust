#![no_std] //dont link to any standard Rust library.
#![no_main] //Disables all rust level entry points.

use core::panic::PanicInfo;

#[panic_handler] 
// this function is called on panic.
fn panic(_panic: &PanicInfo) -> ! {
loop {}
}

static HELLO: &[u8] = b"Hello Aaditya";

#[no_mangle] // dont mangle the name of this function.
//Entry point as the linker looks for function named _start by default. 
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate(){
        //unsafe tells the compiler that the operations are valid.
        unsafe{
            *vga_buffer.offset(i as isize*2)=byte;
            *vga_buffer.offset(i as isize*2+1)=0xb;
        }
    }
loop {}
}

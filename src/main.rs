#![no_std] //dont link to any standard Rust library.
#![no_main] //Disables all rust level entry points.
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main" ]

mod vga_buffer;
mod serial;

use core::panic::PanicInfo;


#[cfg(not(test))]
#[panic_handler] 
// this function is called on panic.
fn panic(info: &PanicInfo) -> ! {
println!("{}",info );
loop {}
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode{
Success= 0x10,
Failed= 0x11, 
}

pub fn exit_qemu(exit_code: QemuExitCode){
use x86_64::instructions::port::Port;

unsafe{
let mut port = Port::new(0xf4);
port.write(exit_code as u32);
}
}


#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
serial_println!("[Failed] \n");
serial_println!("{}",info);
exit_qemu(QemuExitCode::Failed);
loop{}
}


#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()])
{
    serial_println!("Running tests : {} ",tests.len());

    for test in tests{
        test();
    }

    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion(){
serial_print!("Trivial assertion");
assert_eq!(1,1 );
serial_println!("...[ok]" );
}


#[no_mangle] // dont mangle the name of this function.
//Entry point as the linker looks for function named _start by default. 
pub extern "C" fn _start() -> ! {

    println!("hi how are you !!", );
    
    #[cfg(test)]
    test_main();

    loop {}
}

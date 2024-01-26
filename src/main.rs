#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use os::println;
use bootloader::{BootInfo,entry_point};


entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo ) -> ! {
    
    os::init();
    use os::memory;

    use x86_64::{structures::paging::Translate, VirtAddr};

    println!("Hello Aaditya{}", "!");
    println!("\nHow are you doing 🦀");
    


    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let mapper = unsafe { memory::init(phys_mem_offset) };

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }

    #[cfg(test)]
    test_main();
    
    os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info)
}

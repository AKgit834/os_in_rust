

use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::{GlobalDescriptorTable,Descriptor,SegmentSelector};
use lazy_static::lazy_static;


pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

//TSS was used for task switching in older systems.It is as data structure.
//it is used to store info about a task like location of task's stack , state of processors
//registers.
lazy_static!{
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096*5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            let start_ptr = VirtAddr::from_ptr(unsafe{&STACK});
            let end_ptr = start_ptr + STACK_SIZE;
            end_ptr // because stack grows downward.
        };
        tss
    };

}
//GDT is a system structure which holds diffrent descriptors for diffrent memory segments.It is a
//Data structure for memory management.
//
lazy_static!{
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        //Each entry in GDT is a segment descriptor.A segment Descriptor contains info like base
        //address of segment,limit of segment , access rights etc.
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt,Selectors {code_selector, tss_selector})
    };
}

struct Selectors{
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub fn init(){
    use x86_64::instructions::tables::load_tss;
    use x86_64::instructions::segmentation::{CS, Segment};

    GDT.0.load();
    unsafe{
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
}

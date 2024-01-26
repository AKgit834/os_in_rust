


use x86_64::{structures::paging::PageTable,VirtAddr,};
use x86_64::PhysAddr;
use x86_64::structures::paging::OffsetPageTable;

/// Initialize a new OffsetPageTable.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
/// 'static means that the instance will be valid for complete lifetime of our kernel
pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> 
{
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table,physical_memory_offset)
}

unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable
{
    use x86_64::registers::control::Cr3;

    let (level_4_frame,_) = Cr3::read();
    let phys = level_4_frame.start_address();
    let virt = physical_memory_offset+phys.as_u64() ;
    let mut_page_table_pointer: * mut PageTable = virt.as_mut_ptr();
    &mut *mut_page_table_pointer 
}

pub unsafe fn translate_addr(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr>
{
    translate_addr_inner(addr,physical_memory_offset)
}

fn translate_addr_inner(addr: VirtAddr,physical_memory_offset: VirtAddr) -> Option<PhysAddr> 
{
    use x86_64::structures::paging::page_table::FrameError;
    use x86_64::registers::control::Cr3;
    
    let (level_4_frame,_)=Cr3::read();

    let table_indexes=[
        addr.p4_index(),addr.p3_index(),addr.p2_index(),addr.p1_index()
    ];

    let mut frame = level_4_frame;

    for &index in &table_indexes{
        let virt = physical_memory_offset + frame.start_address().as_u64();
        let table_ptr: *const PageTable = virt.as_ptr();

        let table = unsafe{ &*table_ptr };
        
        let entry = &table[index];
        frame = match entry.frame(){
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("Huge page not supported"),
        };
    }
    Some(frame.start_address() + u64::from(addr.page_offset()))

}


use bootloader::bootinfo::{MemoryMap,MemoryRegionType};


use x86_64::{
    VirtAddr,
    PhysAddr,
    structures::paging::{PageTable,OffsetPageTable,Page,PhysFrame,Mapper,Size4KiB,FrameAllocator}
};

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
    // we are not intrested in flags.read returns frame of highest level page table and flags.
    let (level_4_frame,_) = Cr3::read();
    let phys = level_4_frame.start_address();
    let virt = physical_memory_offset+phys.as_u64() ;
    let mut_page_table_pointer: * mut PageTable = virt.as_mut_ptr();
    &mut *mut_page_table_pointer 
}

pub fn createFrame(page: Page,mapper: &mut OffsetPageTable,frame_allocator: & mut impl FrameAllocator<Size4KiB>)
{
    use x86_64::structures::paging::PageTableFlags as Flags;

    let frame=PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags=Flags::PRESENT | Flags::WRITABLE;
    
    let map_to_res=unsafe{
        mapper.map_to(page,frame,flags,frame_allocator)
    };
    map_to_res.expect("map_to failed").flush();
}

//
// pub struct EmptyFrameAllocator;
//
// unsafe impl FrameAllocator<Size4KiB> for 
// EmptyFrameAllocator{
//   fn allocate_frame(&mut self) -> Option<PhysFrame>{
//       None
//   }
//
// }
//A FrameAllocator that returns usable frames from the bootloader's memory map.
pub struct BootInfoFrameAllocator{
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator{
    pub unsafe fn init(memory_map: &'static MemoryMap)->Self{
        BootInfoFrameAllocator{memory_map,next:0}
    }
}

impl BootInfoFrameAllocator{
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame>{
        //get usable regions from memory map
        let regions = self.memory_map.iter();
        let usable_regions=regions.filter(|r| r.region_type == MemoryRegionType::Usable);
        //map each region to its addr range.
        let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());
        //transform to ans iterator of frame start addr.
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        // create PhysFrame types from the start addr.
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator{
    fn allocate_frame(&mut self) -> Option<PhysFrame>{
        let frame=self.usable_frames().nth(self.next);
        self.next+=1;
        frame
    }
}


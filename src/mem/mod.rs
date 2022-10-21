use x86_64::{
    structures::paging::{
        PageTable,
        OffsetPageTable,
        Translate,
        FrameAllocator,
        Size4KiB,
        PhysFrame,
        Page,
        PageTableFlags,
        Mapper
    },
    registers::control::Cr3,
    VirtAddr,
    PhysAddr
};

use crate::info::phys_offset;

mod allocator;


static mut MAPPER : Option<OffsetPageTable> = None;


pub fn init() -> () {
    let offset        = phys_offset();
    let level_4_table = active_level_4_table(offset);
    unsafe {MAPPER = Some(OffsetPageTable::new(level_4_table, offset));}
}



fn active_level_4_table(phys_offset : VirtAddr) -> &'static mut PageTable {
    let (level_4_table_frame, _) = Cr3::read();

    let phys                   = level_4_table_frame.start_address();
    let virt                   = phys_offset + phys.as_u64();
    let table : *mut PageTable = virt.as_mut_ptr();

    return unsafe {&mut *table};
}



pub fn translate_addr(addr : VirtAddr) -> Option<PhysAddr> {
    return (unsafe {MAPPER.as_ref().unwrap()}).translate_addr(addr);
}

pub fn create_mapping(
    page      : Page,
    allocator : &mut impl FrameAllocator<Size4KiB>
) {
    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
    let map   = unsafe {MAPPER.as_mut().unwrap().map_to(page, frame, flags, allocator)};
    map.expect("`map_to` call failed.").flush();
}



pub fn new_allocator() -> allocator::MemFrameAllocator {
    return allocator::MemFrameAllocator::init();
}

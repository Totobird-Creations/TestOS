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
        Mapper,
        mapper::MapToError
    },
    registers::control::Cr3,
    VirtAddr,
    PhysAddr
};

use crate::info::phys_offset;

use self::allocator::{
    MemFrameAllocator,
    ALLOCATOR,
    HEAP_START,
    HEAP_SIZE
};

pub mod allocator;


static mut MAPPER          : Option<OffsetPageTable>              = None;
static mut FRAME_ALLOCATOR : Option<MemFrameAllocator> = None;


pub fn init() -> Result<(), MapToError<Size4KiB>> {
    let offset        = phys_offset();
    let level_4_table = active_level_4_table(offset);
    unsafe {MAPPER          = Some(OffsetPageTable::new(level_4_table, offset));}
    unsafe {FRAME_ALLOCATOR = Some(MemFrameAllocator::init());}
    init_heap()?;
    return Ok(());
}
fn init_heap() -> Result<(), MapToError<Size4KiB>> {
    let range = {
        let heap_start      = VirtAddr::new(HEAP_START as u64);
        let heap_end        = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page   = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };
    let allocator = unsafe {FRAME_ALLOCATOR.as_mut().unwrap()};
    for page in range {
        let frame = allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {MAPPER.as_mut().unwrap().map_to(page, frame, flags, allocator)?.flush()}
    }
    unsafe {ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE)}
    return Ok(());
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

pub fn create_mapping(page : Page) {
    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
    let map   = unsafe {MAPPER.as_mut().unwrap().map_to(page, frame, flags, FRAME_ALLOCATOR.as_mut().unwrap())};
    map.expect("`map_to` call failed.").flush();
}

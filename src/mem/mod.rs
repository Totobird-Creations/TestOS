use x86_64::{
    structures::paging::PageTable,
    registers::control::Cr3,
    VirtAddr,
};


pub unsafe fn active_level_4_table(phys_offset : VirtAddr) -> &'static mut PageTable {
    let (level_4_table_frame, _) = Cr3::read();

    let phys                  = level_4_table_frame.start_address();
    let virt                  = phys_offset + phys.as_u64();
    let ptr  : *mut PageTable = virt.as_mut_ptr();

    return &mut *page_table_ptr;
}

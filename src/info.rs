use x86_64::VirtAddr;
use bootloader::{
    BootInfo,
    bootinfo::MemoryMap
};


pub static mut EXPECT_PANIC : bool = false;


static mut INFO        : Option<&'static BootInfo>  = None;
static mut PHYS_OFFSET : Option<VirtAddr>           = None;
static mut MEMORY_MAP  : Option<&'static MemoryMap> = None;


pub fn load(info : &'static BootInfo) {
    let phys_offset = VirtAddr::new(info.physical_memory_offset);
    let memory_map  = &info.memory_map;
    unsafe {
        INFO        = Some(info);
        PHYS_OFFSET = Some(phys_offset);
        MEMORY_MAP  = Some(memory_map);
    }
}


pub fn expect_panic() -> bool {
    return unsafe {EXPECT_PANIC};
}

pub fn phys_offset() -> VirtAddr {
    return unsafe {PHYS_OFFSET}.unwrap();
}

pub fn memory_map() -> &'static MemoryMap {
    return unsafe {MEMORY_MAP.unwrap()};
}

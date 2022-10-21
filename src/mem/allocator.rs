use bootloader::bootinfo::MemoryRegionType;
use x86_64::{
    structures::paging::{
        FrameAllocator,
        Size4KiB,
        PhysFrame
    },
    PhysAddr
};

use crate::info::memory_map;



pub struct MemFrameAllocator {
    next : usize
}

impl MemFrameAllocator {
    pub fn init() -> Self {
        return MemFrameAllocator {
            next : 0
        };
    }
    pub fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        let map            = memory_map();
        let usable_regions = map.iter().filter(|r| r.region_type == MemoryRegionType::Usable);
        let addr_ranges    = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());
        let frame_addrs    = addr_ranges.flat_map(|r| r.step_by(4096));
        return frame_addrs.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)));
    }
}

unsafe impl FrameAllocator<Size4KiB> for MemFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        return frame;
    }
}
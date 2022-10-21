use crate::vga;


pub fn main() {

    vga::print!("Text printed to screen.\n");

    let mut allocator = crate::mem::new_allocator();
    let     page      = x86_64::structures::paging::Page::containing_address(x86_64::VirtAddr::new(0));
    crate::mem::create_mapping(page, &mut allocator);

    let page_ptr : *mut u64 = page.start_address().as_mut_ptr();
    unsafe {page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e);}

    vga::colour!(Red, Cyan);
    vga::print!("Hello ");
    vga::colour!(Blue, Green);
    vga::print!("World!");
    vga::colour!();
    vga::print!("\n");

    vga::colour!(Pink, Magenta);
    vga::print!("Third line.");
    vga::colour!();
    vga::print!("\n");

}
